use std::{fs::File, path};
use rayon::prelude::*;


fn main() {


     let directories = 
     ["003e3ef8-f3ff-41b6-9d28-18cc31f6ad47",
     "02fba391-90c2-4eef-85df-cd37a0b90758",
     "141e0a1a-f6a4-4f00-b669-8a9e194dbcc8",
     "2196f184-486f-44c1-adce-e5b36c799941",
     "280ae5b3-2172-428d-8f7a-c7fca78d6ec0",
     "2997824c-0fb0-4c9a-aa5f-9d06ab568196",
     "2ef61607-188b-4183-bbd4-9a43778c18bf",
     "3ac8725c-d426-4d93-9c29-2890142658cc",
     "435548c0-c82e-4e61-b873-17d7038461fa",
     "4dec2b81-4ce8-43ad-8d81-7758057d27cf",
     "531316d9-63af-4e71-9609-ed59eff4d353",
     "79cef60b-7485-403e-8d9d-fabe26c487d6",
     "7a48cb3b-8b89-4d29-aaa1-2a77c018b196",
     "8e948913-bd9e-4168-8495-8b4dfd0a027e",
     "9060a026-4559-41a6-bba8-93c1f85a791d",
     "a22536d2-f317-4ef8-9ffb-eb0381fc68d4",
     "af940939-3bf1-4ede-bd9f-650276ec63fc",
     "beb08efd-7118-4981-a2fd-9ecfc7da509b",
     "beb3a8a9-0c6a-4b38-ad17-0d3218e22afe",
     "c110a5ca-0d90-464a-a249-23827cf69bee",
     "c4d9768a-3903-4874-bc39-ae53d9a13959",
     "c89fd4df-fc6f-4868-92b5-4f4eb5dbc280",
     "cb6fbc80-9e34-479e-900e-1bfd193e6588",
     "cc93aa28-a118-47a6-afec-2e7da4abe03b",
     "ce0ac342-80d0-4234-8b70-86623a989418",
     "d12654b0-b112-4d76-a2c4-5b76de76fa01",
     "d7f88f60-6bb1-414d-9c31-0174cd740003",
     "dd8db743-b797-4be7-b370-3b2527833808",
     "f0b9e0b8-4065-45d9-8505-83fea17f440d",
     "fd4132fd-2e4a-43f3-81e4-8acd3a39d7da",
     "fe643149-a6a8-40ba-bfa6-d9aefdc216af"];


     let _ = directories.par_iter()
        .map(|path| {
            println!("delete path {:?}", path);
            std::fs::remove_dir_all(path)
        }).count();
        
}
