use std::collections::{HashSet, HashMap};


pub fn zero_a() -> Vec<i32> {
    unimplemented!()
}



pub fn zero_b() -> Vec<i32> {
    unimplemented!()
}


pub fn zero_c() -> Vec<i32> {
    unimplemented!()
}

pub fn zero_d() -> Vec<String> {
    unimplemented!()
}


pub fn zero_e() -> f64 {
    unimplemented!()
}

pub fn zero_f() -> i32 {
    unimplemented!()
}

pub fn zero_g() -> Vec<String> {
    unimplemented!()
}

pub fn zero_h() -> i32 {
    unimplemented!()
}

pub fn zero_i() -> String {
    unimplemented!()
}

pub fn zero_j() -> HashSet<String> {
    unimplemented!()
}

pub fn zero_k() -> HashMap<i32, Vec<String>> {
    unimplemented!()
}
pub fn zero_l() -> Vec<i32> {
    unimplemented!()
}
pub fn zero_m() -> Vec<String> {
    unimplemented!()
}



// TESTS

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn zero_a_test() {
        assert_eq!(vec![1, 2, 3], zero_a());
    }


    #[test]
    fn zero_b_test() {
        assert_eq!(vec![2, 4, 6, 8, 10], zero_b());
    }

    #[test]
    fn zero_c_test() {
        assert_eq!(vec![0, 7, 14, 21, 28, 35, 42, 49, 56, 63, 70, 77, 84, 91, 98],
                   zero_c());
    }

    #[test]
    fn zero_d_test() {
        assert_eq!(vec!["Caio", "Calpurnio"], zero_d());
    }

    #[test]
    fn zero_e_test() {
        assert_eq!(52.0, zero_e());
    }

    #[test]
    fn zero_f_test() {
        assert_eq!(83166, zero_f());
    }

    #[test]
    fn zero_g_test() {
        assert_eq!(vec!["Caio", "Calpurnio", "Filano", "Mevio", "Sempronio", "Tizio"],
                   zero_g());
    }

    #[test]
    fn zero_h_test() {
        assert!(zero_h() % 41 == 0);
    }

    #[test]
    fn zero_i_test() {
        assert_eq!("Tizio, Caio, Sempronio, Mevio, Filano, Calpurnio", zero_i());
    }

    #[test]
    fn zero_j_test() {

        let set: HashSet<String> = ["Tizio", "Caio", "Sempronio", "Mevio", "Filano", "Calpurnio"]
            .iter()
            .map(|s| String::from(*s))
            .collect();
        assert_eq!(set, zero_j());
    }


    #[test]
    fn zero_k_test() {

        let hash_map: HashMap<i32, Vec<String>> = [(2, vec!["Anna", "Emma", "Sara"]),
                                                   (5, vec!["Carla", "Maria"]),
                                                   (6, vec!["Angela", "Chiara"])]
            .iter()
            .map(|&(i, ref vec)| (i, vec.iter().map(|s| String::from(*s)).collect()))
            .collect();

        assert_eq!(hash_map, zero_k());
    }

    #[test]
    fn zero_l_test() {
        assert_eq!(vec![4, 5, 6, 6, 4, 5, 4], zero_l());
    }

    #[test]
    fn zero_m_test() {
        assert_eq!(vec!["A", "C", "A", "C", "E", "M", "S"], zero_m());
    }

}