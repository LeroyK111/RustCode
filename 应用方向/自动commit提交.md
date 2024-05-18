# 调用大模型生成Git commit message的工具

```
一个由大模型驱动的Git提交消息生成器。它旨在通过自动生成基于您的更改而有意义的提交消息，帮助您节省时间。非常适合独立项目或团队协作，可以使您的提交日志整洁易读 Commit Crafter

Installation

cargo install --locked commit_crafter In the git project, install the prepare-commit-msg hook and set up the OpenAI API key to use it. If it is the first time installing and using it.

commit_crafter install After executing the installation command, you must first set up a key in order to use it normally.

commit_crafter config set openai_api_keyOptions

openai api key
commit_crafter config set openai_api_key

openai url
commit_crafter config set openai_url

openai model
commit_crafter config set openai_model

prompt language
commit_crafter config set user_language

get config options
commit_crafter config get

get all config options
commit_crafter config list

The default file path is $HOME/.config/commit_crafter/config.toml

Usage

After correctly installing the hook, execute "git commit -a" in the git project. In the temporary Vim editor interface that opens, there will be generated commit information. The prerequisite is that all files have been staged for commit.
```