use stylist::Style;



pub struct Styles {
    div: Style,
}   

impl Styles {
    pub fn new() -> Self {
        Self {
            div: Self::input_div()
        }   
    }
    
    pub fn div(&self) -> String {
        self.div.get_class_name().to_string()
    }
}



// ====================== Stylesheet Definitions ======================
impl Styles {
    fn input_div() -> Style {
       stylist::style!(
        r#"
            display: flex;
            justify-content: space-between;
        "#
        ).expect("Failed to mount style")
    }
}


