#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::str::FromStr;

component! {
    example_wrangle_aggregate, contracts(list_triple)
    inputs(),
    inputs_array(input: list_triple),
    outputs(output: list_triple),
    outputs_array(),
    option(),
    acc(),
    fn run(&mut self) -> Result<()> {
        for ins in try!(self.ports.get_input_selections("input"))
        {
            println!("Chunk completed!");
            let mut ip = try!(self.ports.recv_array("input", &ins));
            let chunk_reader: list_triple::Reader = try!(ip.read_contract());
            let input_triple = try!(chunk_reader.get_triples());

            let mut ip_acc = try!(self.ports.recv("acc"));
            let acc_reader: list_triple::Reader = try!(ip_acc.read_contract());
            let acc_triple = try!(acc_reader.get_triples());
            let acc_length = acc_triple.len() as u32;
            let input_length = input_triple.len() as u32;

            let mut new_acc_ip = IP::new();
            {
                let ip = new_acc_ip.build_contract::<list_triple::Builder>();
                let mut new_acc_triple = ip.init_triples(acc_length + input_length);
                let first = try!(input_triple.get(0).get_first());
                let mut i :u32 = 0;
                for i in 0..input_triple.len() {
                    let second = try!(input_triple.get(i).get_second());
                    let third = try!(input_triple.get(i).get_third());
                    new_acc_triple.borrow().get(i).set_first(first);
                    new_acc_triple.borrow().get(i).set_second(second);
                    new_acc_triple.borrow().get(i).set_third(third);
                }
                for i in 0..acc_triple.len() {
                    let second = try!(acc_triple.get(i).get_second());
                    let third = try!(acc_triple.get(i).get_third());
                    new_acc_triple.borrow().get(i+input_length).set_first(first);
                    new_acc_triple.borrow().get(i+input_length).set_second(second);
                    new_acc_triple.borrow().get(i+input_length).set_third(third);
                }
            }
            try!(self.ports.send("acc", new_acc_ip));
        }

        let mut ip_acc = try!(self.ports.recv("acc"));
        let acc_reader: list_triple::Reader = try!(ip_acc.read_contract());
        let acc_triple = try!(acc_reader.get_triples());

        let mut large_sized_bean_counter = HashMap::new();
        for i in 0..acc_triple.len() {
            let bean = large_sized_bean_counter.entry(try!(acc_triple.get(i).get_second())).or_insert(0);
            *bean += i32::from_str(try!(acc_triple.get(i).get_third())).unwrap();
        }

        let mut fin_ip = IP::new();
        {
            let ip = fin_ip.build_contract::<list_triple::Builder>();
            let mut fin_triple = ip.init_triples(large_sized_bean_counter.len() as u32);
            let first = try!(acc_triple.get(0).get_first());
            let mut i :u32 = 0;
            for (key,val) in large_sized_bean_counter.iter() {
                fin_triple.borrow().get(i).set_first(first);
                fin_triple.borrow().get(i).set_second(key);
                fin_triple.borrow().get(i).set_third(format!("{}",val).as_str());
                i += 1;
            }
        }
        try!(self.ports.send("output", fin_ip));
        Ok(())
    }
}
