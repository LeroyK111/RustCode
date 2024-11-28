/*
使用状态模式和零大小类型10倍提高Rust api性能
*/

// 如何通过使用泛型、零大小类型及状态模式10倍提高Rust api的性能，同时还能防止API的使用者滥用API。



use std::collections::HashMap;

// 这个项目是构建一个密码管理器的库。首先，我们定义一个名为PasswordManager的结构体：
struct PasswordManager {
    master_pass: String,
    passwords: HashMap<String, String>,
}

/*
它包含两个私有字段：master_pass：管理员密码passwords：用户名与密码的哈希映射
我们还添加了一些函数，使密码管理器可用，代码如下：
*/

impl PasswordManager {
    /*
    
new方法：构造函数unlock方法：解锁密码管理器lock方法：对密码管理器加锁list_passwords方法：列出所有密码add_password方法：向密码管理器添加密码encryption方法：获取密码管理器的加密算法version方法：获取密码管理器的版本信息
这就是PasswordManager的API，让我们看看如何使用它们。在main函数中写入以下代码：
    */
    pub fn new(master_pass: String) -> Self {
        PasswordManager {
            master_pass,
            passwords: Default::default(),
        }
    }

    pub fn unlock(&mut self, master_pass: String) {
        todo!()
    }

    pub fn lock(&mut self) {
        todo!()
    }

    pub fn list_passwords(&self) -> &HashMap<String, String> {
        todo!()
    }

    pub fn add_password(&mut self, name: String, password: String) {
        todo!()
    }

    pub fn encryption(&self) -> String {
        todo!()
    }

    pub fn version(&self) -> String {
        todo!()
    }
}

// fn main() {
//     let mut manager = PasswordManager::new("password123".to_owned());
//     manager.unlock("password123".to_owned());
//     manager.list_passwords();
//     manager.lock();
// }

fn main() {
    let mut manager = PasswordManager::new("password123".to_owned());

    manager.list_passwords();  // 应该先解锁再调用
    manager.unlock("password123".to_owned());
    manager.unlock("password123".to_owned());
    manager.lock();
}

/*

首先，我们创建一个新的密码管理器实例，然后我们通过传入管理员密码来解锁管理器，并列出密码管理器中的密码，最后再次锁定管理器。这就是API应该使用的方式：在列出密码之前，我们必须首先解锁管理器。一旦管理器被锁定，我们就不能再列出或添加密码。然而，没有什么能阻止该API的用户滥用API，我们可以无序地调用这些函数，或者我们也可以连续调用同一函数，即使它没有意义：

正如你所看到的，代码仍然可以编译。这就是一个问题，因为我们不希望API的用户，在使用我们的API时遇到运行时错误或意外结果。密码管理器有两种状态，锁定状态和解锁状态，这些状态具有不同的功能。当密码管理器处于锁定状态时，你应该能够调用unlock方法，但不能够调用lock方法，list_passwords方法和add_password方法也不可以调用如果密码管理器是未锁定状态时，你应该能够调用lock方法，但不能够调用unlock方法，list_passwords方法和add_password方法应该可以调用。还有一些功能，应该能够在两种状态下调用，在这个例子中是encryption和version方法现在我们知道了问题是什么，让我们来看看一些解决方案。
*/


/*
!解决方案一第一个解决方案是向密码管理器添加一些额外的状态。在这个例子中，我们添加一个名为locked的新字段，它是一个布尔值：
struct PasswordManager {
    locked: bool,
    master_pass: String,
    passwords: HashMap<String, String>,
}

impl PasswordManager {
    pub fn new(master_pass: String) -> Self {
        PasswordManager {
            locked: true,
            master_pass,
            passwords: Default::default(),
        }
    }
    ......
}    
*/

// 在构造函数中将locked设置为true，然后，当调用unlock方法时，如果密码与存储的管理员密码匹配，我们可以将locked设置为false。最后，在list_passwords和add_password方法中，我们可以检查locked是否设置为false，如果locked为true，那么我们需要做一些错误处理。我们可以通过两种方式做到这一点，第一种方式是简单的Panic，这并不理想，因为我们不想只是因为用户打错了密码，就让程序崩溃。第二种方式是，更改函数的签名以返回Result类型，如果locked为true，我们可以简单地返回一个Error：

/*
列出所有密码
pub fn list_passwords(&self) -> Result<&HashMap<String, String>, &dyn Error> {
    todo!()
}

    由于几个原因，解决方案一并不理想。首先，它使API变得复杂，list_passwords方法返回的不是一个简单的HashMap，它现在返回一个Result类型，这迫使API的用户理解为什么这个函数可能会出错。其次，无论密码管理器是锁定还是解锁，这个API中的所有方法都显示给了用户。例如，在创建新的密码管理器实例之后，即使密码管理器处于锁定状态，如果尝试检查可用的方法，我们仍然可以看到list_passwords和add_password方法是可用的。如图：

    理想情况下，这些方法不应该列出，除非密码管理器已解锁。当前解决方案的问题是它使用运行时检查，所以在调用list_passwords方法时不会给我们任何编译时错误，我们只会在运行时注意到问题以解决这些问题。下面让我们探索一个编译时检查的解决方案。
*/


/*

!解决方案二该方案不是只使用一个密码管理器结构体，我们使用两个结构体，一个锁定的密码管理器结构体和一个解锁的密码管理器结构体：

struct LockedPasswordManager {
    master_pass: String,
    passwords: HashMap<String, String>,
}

struct UnlockedPasswordManager {
    master_pass: String,
    passwords: HashMap<String, String>,
}

impl LockedPasswordManager {
    pub fn new(master_pass: String) -> Self {
        LockedPasswordManager {
            master_pass,
            passwords: Default::default(),
        }
    }

    // 解锁密码管理器
    pub fn unlock(&self, master_pass: String) -> UnlockedPasswordManager {
        UnlockedPasswordManager { 
            master_pass: self.master_pass.clone(), 
            passwords: self.passwords.clone(), 
        }
    }

    // 获取密码管理器的加密算法
    pub fn encryption(&self) -> String {
        todo!()
    }

    // 获取密码管理器的版本信息
    pub fn version(&self) -> String {
        todo!()
    }
}

    在LockedPasswordManager的实现块中，包含一个构造函数，一个返回UnlockedPasswordManager结构体的unlock方法，它还包含encryption方法和version方法。同时，我们将lock方法、list_passwords方法和add_password方法移动到了UnlockedPasswordManager实现块中：



impl UnlockedPasswordManager {
    // 对密码管理器加锁
    pub fn lock(&self) -> LockedPasswordManager {
        LockedPasswordManager { 
            master_pass: self.master_pass.clone(), 
            passwords: self.passwords.clone()
        }
    }

    // 列出所有密码
    pub fn list_passwords(&self) -> &HashMap<String, String> {
        todo!()
    }

    // 向密码管理器添加密码
    pub fn add_password(&mut self, name: String, password: String) {
        todo!()
    }

    // 获取密码管理器的加密算法
    pub fn encryption(&self) -> String {
        todo!()
    }

    // 获取密码管理器的版本信息
    pub fn version(&self) -> String {
        todo!()
    }
}

lock方法返回LockedPasswordManager结构体，UnlockedPasswordManager还必须实现encryption和version方法，因为这些方法是通用的。现在我们的API已经更新了，让我们看看如何在Main中使用它：
首先，我们修改PasswordManager为LockedPasswordManager，这将自动给出编译时错误，因为list_passwords方法和lock方法在LockedPasswordManager上不可用。这很好，因为用户只能访问在锁定状态下有意义的方法。修改main函数的代码，如下：


fn main() {
    let mut manager = LockedPasswordManager::new("password123".to_owned());
    let manager =  manager.unlock("password123".to_owned());
    manager.list_passwords(); 
    manager.lock();
}

    很好，现在我们可以防止API的用户在编译时误用它了。但由于几个原因，这种解决方案仍然不理想。注意，在这两个结构体中有相当多的重复代码，包含相同的字段。这两个结构体也必须实现两个状态之间共同的功能，encryption方法和version方法。我们希望保留编译时检查，但不需要所有这些重复的代码。下面我们使用泛型和零大小类型来实现这一点。
    */


    /*
    !解决方案三我们重新定义结构体：use std::{collections::HashMap, marker::PhantomData};

struct Locked;
struct Unlocked;

struct PasswordManager<State = Locked> {
    master_pass: String,
    passwords: HashMap<String, String>,
    state: PhantomData<State>,
}

首先，我们回到了使用一个名为PasswordManager的结构体，在结构体中添加了一个名为State的新字段，类型为PhantomData。同时还创建了两个单元结构体Locked和Unlocked，来表示锁定和解锁状态。接下来，我们还向PasswordManager结构体添加一个泛型参数State，并将其默认值设置为Locked。添加泛型形参会导致一个问题，我们必须在结构体的某个地方使用泛型形参，问题是我们并不关心这个泛型参数，我们只使用它来创建不同的类型。这就是PhantomData的来源，PhantomData是一种零大小类型，只是用于标记。在编译时，这个字段实际上会被优化掉，这就是为什么PhantomData被称为零大小类型，因为它不占用空间。我们在实例化PasswordManager时，必须将这个泛型形参替换为一个具体类型，默认为Locked结构体，也可能是Unlocked结构体。这么做这是有益的，因为锁定的密码管理器不等于解锁的密码管理器，这是两种不同的类型，这意味着我们可以在每种类型上实现不同的方法。现在我们已经重新定义了PasswordManager，让我们来修改实现块：

impl PasswordManager<Locked> {
    // 解锁密码管理器
    pub fn unlock(&self, master_pass: String) -> PasswordManager<Unlocked> {
        PasswordManager {
            master_pass: self.master_pass.clone(),
            passwords: self.passwords.clone(),
            state: PhantomData::<Unlocked>,
        }
    }
}

impl PasswordManager<Unlocked> {
    // 对密码管理器加锁
    pub fn lock(&self) -> PasswordManager<Locked> {
        PasswordManager {
            master_pass: self.master_pass.clone(),
            passwords: self.passwords.clone(),
            state: PhantomData::<Locked>,
        }
    }

    // 列出所有密码
    pub fn list_passwords(&self) -> &HashMap<String, String> {
        &self.passwords
    }

    // 向密码管理器添加密码
    pub fn add_password(&mut self, name: String, password: String) {
        self.passwords.insert(name, password);
    }
}

impl<State> PasswordManager<State> {
    // 获取密码管理器的加密算法
    pub fn encryption(&self) -> String {
        todo!()
    }

    // 获取密码管理器的版本信息
    pub fn version(&self) -> String {
        todo!()
    }
}

impl PasswordManager {
    pub fn new(master_pass: String) -> Self {
        PasswordManager {
            master_pass,
            passwords: Default::default(),
            state: Default::default(),
        }
    }
}

我们已经完成了密码管理器的实现，让我们继续修改Main函数：fn main() {
    let manager = PasswordManager::new("password123".to_owned());
    let manager = manager.unlock("password123".to_owned());
    manager.list_passwords();
    manager.lock();
}

恭喜，现在我们知道如何使用泛型、零大小类型及状态模式在提高Rust api性能的同时还能防止API的使用者滥用API。
    */