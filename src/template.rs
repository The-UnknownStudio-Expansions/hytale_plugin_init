pub fn apply_context<T: crate::template_context::TemplateContext>(
    file_context: T,
) -> tera::Context {
    let mut tera_ctx = tera::Context::new();
    tera_ctx.insert("self", &file_context);
    tera_ctx
}
