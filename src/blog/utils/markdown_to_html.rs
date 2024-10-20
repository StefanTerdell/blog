pub fn markdown_to_html(content: &String) -> String {
    let mut options = comrak::Options::default();

    options.extension.underline = true;
    options.extension.table = true;
    options.extension.footnotes = true;
    options.extension.tasklist = true;

    let mut plugins = comrak::Plugins::default();
    let adapter = comrak::plugins::syntect::SyntectAdapterBuilder::new()
        .syntax_set(
            syntect::dumps::from_uncompressed_data(include_bytes!("../../../syntaxes.bin"))
                .expect("Failed loading syntax set"),
        )
        .theme("base16-ocean.dark")
        .build();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let result = comrak::markdown_to_html_with_plugins(
        &content.replace("```typescript", "```TypeScriptReact"),
        &options,
        &plugins,
    );

    result
}
