#![crate_name = "rust_rc4"]
#![crate_type = "lib"]


pub struct Rc4 {
    i: u8,
    j: u8,
    state: [u8; 256]
}

impl Rc4 {
    
    ///Key-scheduling algorithm 
    pub fn ksa(key: &[u8]) -> Rc4 {
        //Check if key length is sufficent
        assert!(key.len() >= 5 && key.len() <= 256);
        let mut rc4 = Rc4 { i: 0, j: 0, state: [0; 256] };
        
        for i in 0..256 {
            rc4.state[i] = i as u8;
        }

        for i in 0..256 {
            rc4.j = rc4.j.wrapping_add(rc4.state[i]).wrapping_add(key[i % key.len()]);
            rc4.state.swap(i, rc4.j as usize);
        }

        //Both i and j should be 0 when prga is called 1st time
        rc4.j = 0;

        rc4
    }

    ///Pseudo-random generation algorithm
    fn prga(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1u8);
        self.j = self.j.wrapping_add(self.state[self.i as usize]);
        self.state.swap(self.i as usize, self.j as usize);
        self.state[self.state[self.i as usize].wrapping_add(self.state[self.j as usize]) as usize]
    }

    ///Apply Rc4 to given input slice
    pub fn apply(&mut self, input: &[u8], output: &mut [u8]) {
        assert!(input.len() == output.len());
        for (i, x) in input.iter().enumerate() {
            output[i] = self.prga() ^ *x;
        }
    }

}

#[cfg(test)]
mod test {
    extern crate rustc_serialize;
    use test::rustc_serialize::hex::FromHex;
    use super::Rc4;

    struct Test {
        key: &'static str,
        input: &'static str,
        output: &'static str
    }

    fn tests() -> Vec<Test> {
        vec![
            Test {
                key: "0102030405",
                input:  "00000000000000000000000000000000",
                output: "b2396305f03dc027ccc3524a0a1118a8"
            },
            Test {
                key: "01020304050607",
                input:  "00000000000000000000000000000000",
                output: "293f02d47f37c9b633f2af5285feb46b"
            },
            Test {
                key: "0102030405060708",
                input:  "00000000000000000000000000000000",
                output: "97ab8a1bf0afb96132f2f67258da15a8"
            }
        ]
    }
    #[test]
    fn it_works() {
        let tests = tests();
        for t in tests.iter() {

            let mut rc4 = Rc4::ksa(t.key.from_hex().unwrap().as_slice());
            let mut output = [0; 16];

            rc4.apply(
                t.input.from_hex().unwrap().as_slice(), &mut output
            );
            
            assert!(t.output.from_hex().unwrap().as_slice() == &output);
        }
    }
}