import yaml

def replace_heading(line: str) -> str:
	if line:
		if line[0] == '#':
			heading_level = 0
			while line[heading_level] == '#':
				heading_level += 1
			subs = (heading_level-1)*"sub"
			line = f"\\{subs}section{{{line[heading_level:].strip()}}}"
	return line

class Document:
	
	def __init__(self, data: dict, content: str):
		self.data = data
		self.content = content

	def __str__(self):
		return str(self.__dict__)
			
	@classmethod
	def load(cls, input: str):
		(_, metadata, content, footer, _ ) = tuple(input.split("---"))
		metadata = yaml.load(metadata, yaml.SafeLoader)
		footer = yaml.load(footer, yaml.SafeLoader)
		data = {**metadata, **footer}
		return cls(data, content)
	
	def to_latex(self) -> str:
		lines = self.content.split('\n')
		body = list(map(replace_heading, lines))

		preamble = []

		# CLASS
		doc_class = self.data.get("class") if self.data.get("class") else "article"
		preamble += [f"\\documentclass{{{doc_class}}}"]

		# TITLE
		if self.data.get('title'):
			title = self.data.get('title')
			preamble += [f"\\title{{{title}}}"]
			titleline = ["\\maketitle"]
			body = titleline + body

		# DATE
		if self.data.get("date"):
			date = self.data.get("date")
			preamble += [f"\\date{{{date}}}"]

		body = "\n".join(body)
		preamble = "\n".join(preamble)
		output = \
f"""{preamble}
\\begin{{document}}
{body}
\\end{{document}}
"""
		return output

with open("test.mr") as file:
	content = file.read()
	doc = Document.load(content)
	doc.to_latex()
	print(doc.to_latex())
	
