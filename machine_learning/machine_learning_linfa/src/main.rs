use linfa::prelude::*;
use linfa::traits::Fit;
use linfa_linear::LinearRegression;
use ndarray::{ArrayBase, OwnedRepr};
use polars::prelude::*; // Import polars
/*
使用polar的CSV reader读取CSV文件。

将数据帧打印到控制台以供检查。

从DataFrame中提取“AGE”列作为线性回归的目标变量。将目标列强制转换为Float64(双精度浮点数)，这是机器学习中数值数据的常用格式。

将features DataFrame转换为narray::ArrayBase(一个多维数组)以与linfa兼容。将目标序列转换为数组，这些数组与用于机器学习的linfa库兼容。

使用80-20的比例将数据集分割为训练集和验证集，这是机器学习中评估模型在未知数据上的常见做法。

使用linfa的线性回归算法在训练数据集上训练线性回归模型。

使用训练好的模型对验证数据集进行预测。

计算验证数据集上的R²(决定系数)度量，以评估模型的性能。R²值表示回归预测与实际数据点的近似程度。
*/


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 将制表符定义为分隔符
    let separator = b'\t';

    let df = polars::prelude::CsvReader::from_path("./diabetes_file.csv")?
        .infer_schema(None)
        .with_separator(separator)
        .has_header(true)
        .finish()?;

    println!("{:?}", df);

    // 提取并转换目标列
    let age_series = df.column("AGE")?.cast(&DataType::Float64)?;
    let target = age_series.f64()?;

    println!("Creating features dataset");

    let mut features = df.drop("AGE")?;

    // 遍历列并将每个列强制转换为Float64
    for col_name in features.get_column_names_owned() {
        let casted_col = df
            .column(&col_name)?
            .cast(&DataType::Float64)
            .expect("Failed to cast column");

        features.with_column(casted_col)?;
    }

    println!("{:?}", df);

    let features_ndarray: ArrayBase<OwnedRepr<_>, _> =
        features.to_ndarray::<Float64Type>(IndexOrder::C)?;
    let target_ndarray = target.to_ndarray()?.to_owned();
    let (dataset_training, dataset_validation) =
        Dataset::new(features_ndarray, target_ndarray).split_with_ratio(0.80);

    // 训练模型
    let model = LinearRegression::default().fit(&dataset_training)?;

    // 预测
    let pred = model.predict(&dataset_validation);

    // 评价模型
    let r2 = pred.r2(&dataset_validation)?;
    println!("r2 from prediction: {}", r2);

    Ok(())
}

/*
执行cargo run，运行结果如下：
shape: (442, 11)
┌─────┬─────┬──────┬───────┬───┬──────┬────────┬─────┬─────┐
│ AGE ┆ SEX ┆ BMI  ┆ BP    ┆ … ┆ S4   ┆ S5     ┆ S6  ┆ Y   │
│ --- ┆ --- ┆ ---  ┆ ---   ┆   ┆ ---  ┆ ---    ┆ --- ┆ --- │
│ i64 ┆ i64 ┆ f64  ┆ f64   ┆   ┆ f64  ┆ f64    ┆ i64 ┆ i64 │
╞═════╪═════╪══════╪═══════╪═══╪══════╪════════╪═════╪═════╡
│ 59  ┆ 2   ┆ 32.1 ┆ 101.0 ┆ … ┆ 4.0  ┆ 4.8598 ┆ 87  ┆ 151 │
│ 48  ┆ 1   ┆ 21.6 ┆ 87.0  ┆ … ┆ 3.0  ┆ 3.8918 ┆ 69  ┆ 75  │
│ 72  ┆ 2   ┆ 30.5 ┆ 93.0  ┆ … ┆ 4.0  ┆ 4.6728 ┆ 85  ┆ 141 │
│ 24  ┆ 1   ┆ 25.3 ┆ 84.0  ┆ … ┆ 5.0  ┆ 4.8903 ┆ 89  ┆ 206 │
│ …   ┆ …   ┆ …    ┆ …     ┆ … ┆ …    ┆ …      ┆ …   ┆ …   │
│ 47  ┆ 2   ┆ 24.9 ┆ 75.0  ┆ … ┆ 5.0  ┆ 4.4427 ┆ 102 ┆ 104 │
│ 60  ┆ 2   ┆ 24.9 ┆ 99.67 ┆ … ┆ 3.77 ┆ 4.1271 ┆ 95  ┆ 132 │
│ 36  ┆ 1   ┆ 30.0 ┆ 95.0  ┆ … ┆ 4.79 ┆ 5.1299 ┆ 85  ┆ 220 │
│ 36  ┆ 1   ┆ 19.6 ┆ 71.0  ┆ … ┆ 3.0  ┆ 4.5951 ┆ 92  ┆ 57  │
└─────┴─────┴──────┴───────┴───┴──────┴────────┴─────┴─────┘
Creating features dataset
shape: (442, 11)
┌─────┬─────┬──────┬───────┬───┬──────┬────────┬─────┬─────┐
│ AGE ┆ SEX ┆ BMI  ┆ BP    ┆ … ┆ S4   ┆ S5     ┆ S6  ┆ Y   │
│ --- ┆ --- ┆ ---  ┆ ---   ┆   ┆ ---  ┆ ---    ┆ --- ┆ --- │
│ i64 ┆ i64 ┆ f64  ┆ f64   ┆   ┆ f64  ┆ f64    ┆ i64 ┆ i64 │
╞═════╪═════╪══════╪═══════╪═══╪══════╪════════╪═════╪═════╡
│ 59  ┆ 2   ┆ 32.1 ┆ 101.0 ┆ … ┆ 4.0  ┆ 4.8598 ┆ 87  ┆ 151 │
│ 48  ┆ 1   ┆ 21.6 ┆ 87.0  ┆ … ┆ 3.0  ┆ 3.8918 ┆ 69  ┆ 75  │
│ 72  ┆ 2   ┆ 30.5 ┆ 93.0  ┆ … ┆ 4.0  ┆ 4.6728 ┆ 85  ┆ 141 │
│ 24  ┆ 1   ┆ 25.3 ┆ 84.0  ┆ … ┆ 5.0  ┆ 4.8903 ┆ 89  ┆ 206 │
│ …   ┆ …   ┆ …    ┆ …     ┆ … ┆ …    ┆ …      ┆ …   ┆ …   │
│ 47  ┆ 2   ┆ 24.9 ┆ 75.0  ┆ … ┆ 5.0  ┆ 4.4427 ┆ 102 ┆ 104 │
│ 60  ┆ 2   ┆ 24.9 ┆ 99.67 ┆ … ┆ 3.77 ┆ 4.1271 ┆ 95  ┆ 132 │
│ 36  ┆ 1   ┆ 30.0 ┆ 95.0  ┆ … ┆ 4.79 ┆ 5.1299 ┆ 85  ┆ 220 │
│ 36  ┆ 1   ┆ 19.6 ┆ 71.0  ┆ … ┆ 3.0  ┆ 4.5951 ┆ 92  ┆ 57  │
└─────┴─────┴──────┴───────┴───┴──────┴────────┴─────┴─────┘
r2 from prediction: 0.15937814745521017
*/