use std::collections::HashMap;


/**
 * Each customer may have different needs, for instance
 * a Driver or Delivery jobs customer may need delivery jobs script file
 * which has a street address but no salary, however
 * the online jobs sites like qkids or tutree (vipkid) may require
 * a salary and a "REMOTE" location, so if we need the site to
 * be able to serve multiple customers at the same time in the same domain
 * we need th allow for multple script files in the scripts folder.
 *
 * Asumming no custom script is found, we use the default script
 * which acts as a fallback
 * */

fn get_script_template_hashmap() -> HashMap<String,String> {
    let mut scripts:HashMap<String,String> = HashMap::new();
    scripts.insert(
        "default".to_string(),
        String::from(include_str!("../scripts/default.html")),
    );
    scripts.insert(
        "vipkid".to_string(),
        String::from(include_str!("../scripts/vipkid.html")),
    );
    scripts.insert(
        "doordash".to_string(),
        String::from(include_str!("../scripts/doordash.html")),
    );
    scripts.insert(
        "instacart".to_string(),
        String::from(include_str!("../scripts/instacart.html")),
    );

    scripts.insert(
        "qkids".to_string(),
        String::from(include_str!("../scripts/qkids.html")),
    );

    scripts
}

/*
 * get_script_template: takes as input a script_template 
 * (which comes from MySQL's site.script_template field
 * and returns the bytes from the scripts folder.
 * the Scripts folder holds the contents of the JSON Schema.
 * It is important that this function *always* return a valid
 * schema, even when the new script_template isn't valid, so that
 * in the event of a database change, the program still generate valid
 * JobPostings.
 * */

pub fn get_script_template(template:&str) -> String {
    // hashmap
    let scripts = get_script_template_hashmap();
    
    match scripts.get(template) {
        Some(template_contents) => template_contents.clone(),
        None => {
            println!("get_script_template(): can't find script_template:'{}' using default", template);
            //return default.
            String::from("DEFAULT") // site2json_schema
        } ,
    }

}

