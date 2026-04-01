# 開発・Git運用ルール

## ユーザー設定 (コラボマーク抑止)
以下の情報をローカルGitで設定すること。
- URL: [https://github.com/r-522/Certification]
- ユーザ名: [r-522]
- メアド: [ikere105@gmail.com]

## コミット規約
プレフィックス(feat/fix/docs/refactor/style等)を使用し「何故・何をしたか」を日本語で簡潔に記載すること。

## Gitignore定義
以下の項目は不要ファイルとして`.gitignore`に必ず含めること。
.gitignore
.claude
node_modules/
dist/
target/
.env
.prompt.txt

## 依存・インストール関連
何も本PCにインストールせずに進めること。

## 許可・認証
基本的にすべて承認して進めてください。