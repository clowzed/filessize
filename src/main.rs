use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;


fn main() 
{
    let files_ = std::env::args().collect::<Vec<_>>()[1..].to_vec();
    
    let mut files = Vec::new();
    
    for file in files_.iter()
    {  
        match glob::glob(file)
        {
            Ok(paths) => 
            {
                for path in paths.flatten()
                {
                    files.push(path.as_os_str().to_str().unwrap().to_string());
                }
            },
            Err(_) => 
            {
                println!("Bad pattern: {file}");
            }
        }
    }
    

    let mut total: usize = 0;

    let mut table = Table::new();
    
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["File name", "File size"]);

    for file in files.iter() 
    {
        match std::fs::metadata(file) 
        {
            Ok(meta) => 
            {
                let filesize = kib::format(meta.len());
                table.add_row(vec![
                    Cell::new(file),
                    Cell::new(&filesize).fg(comfy_table::Color::Green),
                ]);
                total += meta.len() as usize;
            }
            Err(_) => 
            {
                table.add_row(vec![
                    Cell::new(file),
                    Cell::new("Failed to retrive metadata for file").fg(comfy_table::Color::Red),
                ]);
            }
        }
    }
    let total_size = kib::format(total as u128);

    table.add_row(vec![
        Cell::new("Total size"),
        Cell::new(total_size).fg(comfy_table::Color::Blue),
    ]);
    println!("{table}");
}
