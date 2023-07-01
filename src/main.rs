    use std::default;
    use std::env;
    use std::fs;

#[derive(Default,Debug)]
struct Directory{
    file_names : Vec<String>, 
    extensions : Vec<String>,
}

impl Directory{
    fn push(&mut self,tuple : (String,String)){
        self.file_names.push(tuple.0);
        self.extensions.push(tuple.1);
    }
}

fn FileNameParser(file_name:std::ffi::OsString) -> (String,String){
    let aux = file_name.into_string().unwrap();
    let mut splited = aux.split(".");
    let name : String =  String::from(splited.next().unwrap_or(""));
    let extension : String = String::from(splited.next().unwrap_or(""));

    return (name,extension)
}

fn main(){
// Iniicializando váriaveis.
let mut actual_dir = Directory::default();
let file_extensions = ["pdf","docx","txt","jpg","xlsx",
                                   "mp3","mp4","png","zip","py","c","exe",
                                   "cpp","jpeg"
                                  ]; 
let input_args : Vec<String> = env::args().collect();
if input_args.len() < 2 {
    eprintln!("Sem Argumentos"); return};

let input_path = input_args[1].clone();

// Processando o diretorio
let dir = fs::read_dir(&input_path).unwrap();

for file in dir{
    match file{
        Ok(file) => {
            if file.file_type().unwrap().is_file(){
                let aux = FileNameParser(file.file_name());
                actual_dir.push(aux);
            }    
        }
        Err(err) => println!("Erro"),
    }
}

//Fazendo o processamento dos arquivos que devem ser movidos
let mut process_dir = Directory::default();
let mut extensions_found: Vec<String> = Vec::new();

for (index, extension) in actual_dir.extensions.iter().enumerate() {
    if file_extensions.contains(&extension.as_str()) {
        let extension_clone = extension.clone();
        let file_name_clone = actual_dir.file_names[index].clone();
        process_dir.push((file_name_clone,extension_clone));

        if !extensions_found.contains(&extension){
           extensions_found.push(extension.clone()) 
        }
    }
}

/* 
println!("=========FILES_FOUND=========");
for (index, (name, extension)) in process_dir
    .file_names
    .iter()
    .zip(process_dir.extensions.iter())
    .enumerate()
{
    println!("{} - {}.{}", index + 1, name, extension);
}
*/

/* 
println!("=========Extensions_FOUND=========");
for ext in extensions_found.iter(){
    println!("{}",ext);
}
*/

let dir_builder = fs::DirBuilder::new();

//Criando as pastas para onde os arquivos serão movidos

for extension in extensions_found{
        let mut extension_dir = input_path.clone();
        extension_dir.push_str("\\");
        extension_dir.push_str(&extension.as_str());
        let dir_exist = std::path::Path::new(extension_dir.as_str())
        .try_exists()
        .unwrap_or(false);  
        
        if !dir_exist {
                dir_builder.create(extension_dir.as_str());
                      }
    }

for (name,extension) in process_dir.file_names
    .iter()
    .zip(process_dir.extensions.iter()){
        let mut actual_path = input_path.clone();
        let mut new_path = input_path.clone();

        actual_path.push_str("\\");
        actual_path.push_str(name.as_str());
        actual_path.push_str(".");
        actual_path.push_str(extension.as_str());
        
        new_path.push_str("\\");
        new_path.push_str(extension.as_str());
        new_path.push_str("\\");
        new_path.push_str(name.as_str());
        new_path.push_str(".");
        new_path.push_str(extension.as_str());
        
        fs::rename(actual_path,new_path);
    }

}