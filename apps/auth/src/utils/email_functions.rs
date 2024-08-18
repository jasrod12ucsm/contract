pub struct EmailFunctions;

impl EmailFunctions {
    pub fn replace_placeholders(template: String, data: Vec<&str>) -> String {
        let mut result = template;
        let mut data_iter = data.iter();
        
        while let Some(pos) = result.find("{}") {
            if let Some(replacement) = data_iter.next() {
                result.replace_range(pos..pos+2, replacement);
            } else {
                break;
            }
        }
        
        result
    }
}