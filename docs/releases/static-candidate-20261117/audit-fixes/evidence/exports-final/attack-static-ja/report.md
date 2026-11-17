# 耐量子暗号への移行調査レポート

作成日時: 2026-11-14 00:00:00 UTC

対応を確認する候補: 25 / 参考情報: 0 / 調査したファイル: 25

コードや設定の手掛かりをまとめた調査結果です。

|重大度|件数（全検出）|
|---|---:|
|高|25|

## 対応候補の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|
|[SYNTHETIC048](#rule-0) 日本語の説明が未登録のルールです。原文：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="docume...|高|量子計算によるリスクあり|25|25|

## 参考情報の索引

|ルール・説明|重大度|判断の目安|件数|ファイル数|
|---|---|---|---:|---:|

<a id="rule-0"></a>

## SYNTHETIC048

日本語の説明が未登録のルールです。原文：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;

確認すること: ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;

|検出箇所|検出した記述|対応・詳細|
|---|---|---|
|<a id="finding-0"></a>[日本/unsafe-"&lt;0&gt;.py:1:3](#finding-0)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-0)|
|<a id="finding-1"></a>[日本/unsafe-"&lt;10&gt;.py:11:3](#finding-1)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-1)|
|<a id="finding-2"></a>[日本/unsafe-"&lt;11&gt;.py:12:3](#finding-2)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-2)|
|<a id="finding-3"></a>[日本/unsafe-"&lt;12&gt;.py:13:3](#finding-3)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-3)|
|<a id="finding-4"></a>[日本/unsafe-"&lt;13&gt;.py:14:3](#finding-4)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-4)|
|<a id="finding-5"></a>[日本/unsafe-"&lt;14&gt;.py:15:3](#finding-5)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-5)|
|<a id="finding-6"></a>[日本/unsafe-"&lt;15&gt;.py:16:3](#finding-6)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-6)|
|<a id="finding-7"></a>[日本/unsafe-"&lt;16&gt;.py:17:3](#finding-7)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-7)|
|<a id="finding-8"></a>[日本/unsafe-"&lt;17&gt;.py:18:3](#finding-8)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-8)|
|<a id="finding-9"></a>[日本/unsafe-"&lt;18&gt;.py:19:3](#finding-9)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-9)|
|<a id="finding-10"></a>[日本/unsafe-"&lt;19&gt;.py:20:3](#finding-10)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-10)|
|<a id="finding-11"></a>[日本/unsafe-"&lt;1&gt;.py:2:3](#finding-11)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-11)|
|<a id="finding-12"></a>[日本/unsafe-"&lt;20&gt;.py:21:3](#finding-12)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-12)|
|<a id="finding-13"></a>[日本/unsafe-"&lt;21&gt;.py:22:3](#finding-13)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-13)|
|<a id="finding-14"></a>[日本/unsafe-"&lt;22&gt;.py:23:3](#finding-14)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-14)|
|<a id="finding-15"></a>[日本/unsafe-"&lt;23&gt;.py:24:3](#finding-15)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-15)|
|<a id="finding-16"></a>[日本/unsafe-"&lt;24&gt;.py:25:3](#finding-16)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-16)|
|<a id="finding-17"></a>[日本/unsafe-"&lt;2&gt;.py:3:3](#finding-17)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-17)|
|<a id="finding-18"></a>[日本/unsafe-"&lt;3&gt;.py:4:3](#finding-18)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-18)|
|<a id="finding-19"></a>[日本/unsafe-"&lt;4&gt;.py:5:3](#finding-19)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-19)|
|<a id="finding-20"></a>[日本/unsafe-"&lt;5&gt;.py:6:3](#finding-20)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-20)|
|<a id="finding-21"></a>[日本/unsafe-"&lt;6&gt;.py:7:3](#finding-21)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-21)|
|<a id="finding-22"></a>[日本/unsafe-"&lt;7&gt;.py:8:3](#finding-22)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-22)|
|<a id="finding-23"></a>[日本/unsafe-"&lt;8&gt;.py:9:3](#finding-23)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-23)|
|<a id="finding-24"></a>[日本/unsafe-"&lt;9&gt;.py:10:3](#finding-24)|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='...|ルール作成者の確認事項（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; / [周辺コード](#source-24)|

このルールの全箇所に共通する確認・計画（各行で確認済みの事実ではありません）:

- [日本語未登録の対応案（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;](#action-0)

<a id="source-0"></a>

### 日本/unsafe-"&lt;0&gt;.py:1:3

    > 1 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-1"></a>

### 日本/unsafe-"&lt;10&gt;.py:11:3

    > 11 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-2"></a>

### 日本/unsafe-"&lt;11&gt;.py:12:3

    > 12 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-3"></a>

### 日本/unsafe-"&lt;12&gt;.py:13:3

    > 13 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-4"></a>

### 日本/unsafe-"&lt;13&gt;.py:14:3

    > 14 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-5"></a>

### 日本/unsafe-"&lt;14&gt;.py:15:3

    > 15 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-6"></a>

### 日本/unsafe-"&lt;15&gt;.py:16:3

    > 16 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-7"></a>

### 日本/unsafe-"&lt;16&gt;.py:17:3

    > 17 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-8"></a>

### 日本/unsafe-"&lt;17&gt;.py:18:3

    > 18 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-9"></a>

### 日本/unsafe-"&lt;18&gt;.py:19:3

    > 19 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-10"></a>

### 日本/unsafe-"&lt;19&gt;.py:20:3

    > 20 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-11"></a>

### 日本/unsafe-"&lt;1&gt;.py:2:3

    > 2 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-12"></a>

### 日本/unsafe-"&lt;20&gt;.py:21:3

    > 21 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-13"></a>

### 日本/unsafe-"&lt;21&gt;.py:22:3

    > 22 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-14"></a>

### 日本/unsafe-"&lt;22&gt;.py:23:3

    > 23 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-15"></a>

### 日本/unsafe-"&lt;23&gt;.py:24:3

    > 24 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-16"></a>

### 日本/unsafe-"&lt;24&gt;.py:25:3

    > 25 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-17"></a>

### 日本/unsafe-"&lt;2&gt;.py:3:3

    > 3 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-18"></a>

### 日本/unsafe-"&lt;3&gt;.py:4:3

    > 4 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-19"></a>

### 日本/unsafe-"&lt;4&gt;.py:5:3

    > 5 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-20"></a>

### 日本/unsafe-"&lt;5&gt;.py:6:3

    > 6 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-21"></a>

### 日本/unsafe-"&lt;6&gt;.py:7:3

    > 7 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-22"></a>

### 日本/unsafe-"&lt;7&gt;.py:8:3

    > 8 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-23"></a>

### 日本/unsafe-"&lt;8&gt;.py:9:3

    > 9 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

<a id="source-24"></a>

### 日本/unsafe-"&lt;9&gt;.py:10:3

    > 10 </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script> password='[masked-sensitive-value]'

## 対応案の詳細

各検出箇所から、その箇所に該当する対応案だけを参照します。同じ内容は一度だけ掲載します。

<a id="action-0"></a>


#### 日本語未登録の対応案（原文）：&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt; (high)

&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;

1. &lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " & 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;

参考資料:

- javascript:document.title='EXECUTED048'
- data:text/html,&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;
- https://example.invalid/" onmouseover="document.title='EXECUTED048'

変更前 (python)

    </script><script>document.title='EXECUTED048'</script><img src=x onerror="document.title='EXECUTED048'"><svg onload="document.title='EXECUTED048'"> [click](javascript:document.title='EXECUTED048') | ` ``` " & 日😀
    <img src=x onerror="document.title='EXECUTED048'">
    <script>document.title='EXECUTED048'</script>

変更後

    safe


## 棚卸しデータ

- [CBOM](cbom.json): 0
- [Dependency SBOM](dependency-sbom.json): 0

