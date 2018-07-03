use prim::Primitive;

#[derive(Debug)]
pub enum SpecialForm {
    Define {
        ident: String,
        value: Primitive,
    }
}
