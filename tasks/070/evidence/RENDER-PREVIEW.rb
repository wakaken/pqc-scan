# Temporary QA renderer only. No HTML is added to the product report contract.
require 'rdoc'
require 'rdoc/markdown'
require 'rdoc/markup/to_html'
require 'fileutils'
root, destination = ARGV
Dir.glob(File.join(root, '*', '{en,ja}', 'migration.assessment.md')).sort.each do |input|
  case_name = File.basename(File.dirname(File.dirname(input)))
  language = File.basename(File.dirname(input))
  dir = File.join(destination, case_name, language)
  FileUtils.mkdir_p(dir)
  FileUtils.cp(input, dir)
  FileUtils.cp(File.join(File.dirname(input), 'migration.assessment.v1.json'), dir)
  html = RDoc::Markdown.parse(File.read(input)).accept(RDoc::Markup::ToHtml.new(RDoc::Options.new))
  document = <<~HTML
    <!doctype html><html lang="#{language}"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
    <title>#{case_name} — #{language} migration assessment</title>
    <style>body{font:16px/1.65 system-ui,sans-serif;max-width:960px;margin:24px auto;padding:0 18px;color:#172234;background:#fff}h1,h2,h3{line-height:1.3}h2{margin-top:2rem;border-bottom:1px solid #ccd4de;padding-bottom:.5rem}a{color:#125ba2}a:focus-visible{outline:3px solid #cd6800;outline-offset:3px}pre{background:#f3f5f8;padding:12px;white-space:pre-wrap;overflow-wrap:anywhere}code{font-size:.85em;overflow-wrap:anywhere}li{margin:.4em 0}p{overflow-wrap:anywhere}</style>
    <body id="top"><main>#{html}</main></body></html>
  HTML
  File.write(File.join(dir, 'preview.html'), document)
end
puts "RDoc #{RDoc::VERSION}; rendered #{Dir.glob(File.join(destination, '*', '*', 'preview.html')).size} paired previews"
