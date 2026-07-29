pub mod Helper{
    use std::{any::Any, collections::HashMap, process::exit};

    #[derive(Debug,Clone)]
    pub struct CLI<'a>{
        pub args: HashMap<String,&'a dyn Any>,
        flag_map: HashMap<String,(Vec<&'a str>,for<'b> fn (&'b dyn Any) -> &'b dyn Any)>,
        parse_map: HashMap<&'a str,fn (&str) -> &'a dyn Any>,
        pub preproc: Option<fn (Vec<String>) -> Vec<String>>,
        pub help_fxn: fn(),
        pub iter_fxn: Option<fn()>,
        pub postproc: Option<fn (&'a dyn Any) -> &'a dyn Any>,
        pub endlogic: Option<fn()>,
    }
    

    impl<'a> CLI<'a>{

        fn __DEFAULT_PREPROC(mut args: Vec<String>) -> Vec<String>{
            args.remove(0);
            args
        }

        fn __DEFAULT_HELP(){
            eprintln!("Somethign went Wrong!");
            exit(0);
        }

        pub fn new() -> Self{
            Self { args: HashMap::new(), flag_map: HashMap::new(), parse_map: HashMap::new(), preproc:Some(Self::__DEFAULT_PREPROC),help_fxn:Self::__DEFAULT_HELP,iter_fxn:None,postproc:None,endlogic:None}
        }

        pub fn set_preproc(&mut self,fxn: fn (Vec<String>) -> Vec<String>){
            self.preproc = Some(fxn);
        }

        pub fn set_postproc(&mut self,fxn: fn (&'a dyn Any) -> &'a dyn Any){
            self.postproc = Some(fxn);
        }

        pub fn set_help(&mut self,fxn: fn ()){
            self.help_fxn = fxn;
        }

        pub fn set_iter(&mut self,fxn: fn ()){
            self.iter_fxn = Some(fxn);
        }


        pub fn add_flag(&mut self,arg_name:&'a str,flags:(Vec<&'a str>,for<'b> fn(&'b dyn Any) -> &'b dyn Any),default_val: &'a dyn Any){
            self.args.insert(arg_name.to_string(), default_val);
            self.flag_map.insert(arg_name.to_string(),flags);
        } 

        pub fn add_params(&mut self,params: Vec<(&'a str,fn (&str) -> &'a dyn Any,&'a dyn Any)>) {
            for i in &params{
                self.args.insert(i.0.to_string(), i.2);
                self.parse_map.insert(i.0,i.1);
            }
        } 

        pub fn add_flags(&mut self,flags:Vec<(&'a str,(Vec<&'a str>,for<'b> fn(&'b dyn Any) -> &'b dyn Any),&'a dyn Any)>){
            for i in &flags{
                self.args.insert(i.0.to_string(), i.2);
                self.flag_map.insert(i.0.to_string(),i.1.clone());
            }
        } 

        pub fn add_param(&mut self,arg_name:&'a str,cust_str_fxn: fn (&str) -> &'a dyn Any,default_val: &'a dyn Any) {
            self.args.insert(arg_name.to_string(), default_val);
            self.parse_map.insert(arg_name,cust_str_fxn);
        } 


        pub fn add_arg(&mut self,arg_name:&'a str,flags:Option<(Vec<&'a str>,for<'b> fn(&'b dyn Any) -> &'b dyn Any)>,default_val: &'a dyn Any,cust_str_fxn: Option<fn (&str) -> &'a dyn Any>){
            self.args.insert(arg_name.to_string(), default_val);        
            if let Some(x) = flags{
                self.flag_map.insert(arg_name.to_string(), x);
            }else if let Some(z) = cust_str_fxn{
                self.parse_map.insert(arg_name, z);
            }
        }

        pub fn add_args(&mut self,args:Vec<(&'a str,Option<(Vec<&'a str>,for<'b> fn(&'b dyn Any) -> &'b dyn Any)>,&'a dyn Any,Option<fn (&str) -> &'a dyn Any>)>){
            for i in &args{
                self.add_arg(i.0, i.1.clone(), i.2, i.3);
            }
        }

        pub fn Parse_Args(args:Vec<(&'a str,Option<(Vec<&'a str>,for<'b> fn(&'b dyn Any) -> &'b dyn Any)>,&'a dyn Any,Option<fn (&str) -> &'a dyn Any>)>) -> CLI{
            let mut ret = CLI::new();
            ret.add_args(args);
            ret.Parse();
            return ret;
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
                            *p = V.1(*p);
                            if let Some(x) = self.iter_fxn{
                                x();
                            }
                        }
                    }
                }else{
                    for (k,func) in &self.parse_map{
                        if i.starts_with(k){
                            if let Some(p) = self.args.get_mut(i){
                                *p = func(i);
                                if let Some(x) = self.iter_fxn{
                                    x();
                                }
                            }       

                        }
                    }

                }

            }
            if let Some(x) = self.postproc{
                for (_,v) in &mut self.args{
                    *v = x(*v);
                }
            }
            if let Some(x) = self.endlogic{
                x();
            }


        }

        

    }






}