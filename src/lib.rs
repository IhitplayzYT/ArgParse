pub mod Helper{
    use std::{any::Any, collections::HashMap, process::exit};

    #[derive(Debug,Clone)]
    pub struct CLI{
        pub args: HashMap<String,&'static dyn Any>,
        flag_map: HashMap<String,(Vec<&'static str>,fn (&'static dyn Any) -> &'static dyn Any)>,
        parse_map: HashMap<&'static str,fn (&str) -> &'static dyn Any>,
        pub preproc: Option<fn (Vec<String>) -> Vec<String>>,
        pub help_fxn: fn()
    }


    fn __DEFAULT_PREPROC(mut args: Vec<String>) -> Vec<String>{
        args.remove(0);
        args
    }

    fn __DEFAULT_HELP(){
        eprintln!("Somethign went Wrong!");
        exit(0);
    }

    impl CLI{
        pub fn new() -> Self{
            Self { args: HashMap::new(), flag_map: HashMap::new(), parse_map: HashMap::new(), preproc:Some(__DEFAULT_PREPROC),help_fxn:__DEFAULT_HELP}
        }



        pub fn add_arg(&mut self,arg_name:&'static str,flags:Option<(Vec<&'static str>,fn(&'static dyn Any) -> &'static dyn Any)>,default_val: &'static dyn Any,cust_str_fxn: Option<fn (&str) -> &'static dyn Any>){
            self.args.insert(arg_name.to_string(), default_val);        
            if let Some(x) = flags{
                self.flag_map.insert(arg_name.to_string(), x);
            }else if let Some(z) = cust_str_fxn{
                self.parse_map.insert(arg_name, z);
            }
        }

        pub fn Parse(&mut self){
            let mut clargs = std::env::args().collect::<Vec<String>>();
            if let Some(x) = self.preproc{
                clargs = x(clargs);
            }
            for i in &clargs{
                if self.flag_map.contains_key(i){
                    if let Some(p) = self.args.get_mut(i){
                        if let Some(V) = self.flag_map.get(i){
                            *p = V.1(p.clone());
                        }
                    }
                }else{
                    for (k,func) in &self.parse_map{
                        if i.starts_with(k){
                            if let Some(p) = self.args.get_mut(i){
                                *p = func(i);
                            }       

                        }
                    }

                }

            }
        }

        

    }






}