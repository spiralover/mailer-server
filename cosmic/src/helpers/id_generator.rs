use nanoid::nanoid;

pub fn number_generator(len: usize) -> String {
    let alphabet: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    nanoid!(len, &alphabet)
}
