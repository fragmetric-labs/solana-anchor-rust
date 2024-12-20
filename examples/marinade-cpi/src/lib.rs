anchor_gen::generate_cpi_crate!("idl.json");

anchor_lang::declare_id!("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("IDL loaded for program: {:?}", crate::ID);
    }
}