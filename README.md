# James's Markdown

These documents are equivalent:

<table>
<tr>
<th>Mark</th>
<th>LaTeX</th>
</tr>
<tr>
<td>

```
---
version: 0.0
title: A Test Mark Document
class: article
date: 2022-10-06
---

# Here's a heading
Here's some text.

## Here's a subheading
```
</td>
<td>

```
\documentclass{article}
\title{A Test Mark Document}
\date{2022-10-06}

\begin{document}

\maketitle

\section{Here's a heading}
Here's some text.

\subsection{Here's a subheading}
```

</td>
</tr>
</table>
