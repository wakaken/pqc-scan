"""Temporary QA preview of product Markdown; no product dependency or HTML view."""
import json,sys,hashlib,shutil
from pathlib import Path
from markdown_it import MarkdownIt
root,destination=map(Path,sys.argv[1:3])
parser=MarkdownIt('commonmark',{'html':True,'linkify':False,'typographer':False})
style='body{font:16px/1.65 system-ui,sans-serif;max-width:960px;margin:24px auto;padding:0 18px;color:#172234;background:#fff}h1,h2,h3{line-height:1.3}h2{margin-top:2rem;border-bottom:1px solid #ccd4de;padding-bottom:.5rem}a{color:#125ba2}a:focus-visible{outline:3px solid #cd6800;outline-offset:3px}pre{background:#f3f5f8;padding:12px;white-space:pre-wrap;overflow-wrap:anywhere}code{font-size:.85em;overflow-wrap:anywhere}li{margin:.4em 0}p{overflow-wrap:anywhere}'
count=0
for source in sorted(root.glob('*/*/migration.assessment.md')):
 case,language=source.parts[-3:-1];out=destination/case/language;out.mkdir(parents=True,exist_ok=True)
 for name in ['migration.assessment.md','migration.assessment.v1.json']:shutil.copyfile(source.parent/name,out/name)
 body=parser.render(source.read_text())
 html=f'<!doctype html><html lang="{language}"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{case} — {language} migration assessment</title><style>{style}</style><body id="top"><main>{body}</main></body></html>'
 (out/'preview.html').write_text(html);count+=1
print(f'markdown-it-py 3.0.0, CommonMark, html=true/linkify=false/typographer=false; {count} previews')
