// gabka z lancucha o dlugosci b  tworzy nowy lancuch o dlugosci b
// b = r + c to szerokosc,
// dzielimy ciag na dwie czesci o dlugosci r i c, r to przeplywnosc a c to pojemnosc
// r wplywa na wydajnosc funkcji wynikowej, poniewaz wiadomosc będzie przetwarzac r bitow jednoczesnie
// wartosc c wplywa na bezpieczenstwo wynikowej funkcji gabki
// poziom bezpieczenstwa przed atakiem kolizyjnym to 2 ^ (c/2)

// wiadomosc M z wypelnieniem pad tak aby długosc M + pad była podzielna przez r.
// aby osiagnosc bezpieczenstwo na poziomie 256 bitow to b = 2 * 256 + 64 = 576 bitow
// najlepiej jesli f będzie zachowywac sie jak liczba losowa na przyklad funkcja keccak
// sha3-224 oznacza ze wyjście tej funkcji ma 224 bity czyli l = 224

// w zadaniu
// 25 słów, kazde po 64 bity
// faza sciskania nie wymaga uzycia funkcji f
// XOF oznacza ze mozemy wybrac dlugosc wyjscia funkcji (ile bitow)
// 25 slow oznacza 24 rundy


use sha3::{Digest, Sha3_256, Sha3_224, Sha3_384, Sha3_512};

// zadanie 4 do srody




fn hash_vector_to_string(hash_vector: Vec<u8>) -> String{
    hash_vector.into_iter()
        .map(|value| format!("{:0>2x}", value))
        .reduce(|acc, x| {
            format!("{acc}{x}")
        }).unwrap()
}

fn create_hash_string224(s: &str) -> String {
    let mut hasher = Sha3_224::new();
    hasher.update(s);

    let result = hasher.finalize();
    return hash_vector_to_string(result.to_vec());
}

fn create_hash_string256(s: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(s);

    let result = hasher.finalize();
    return hash_vector_to_string(result.to_vec());
}

fn create_hash_string384(s: &str) -> String {
    let mut hasher = Sha3_384::new();
    hasher.update(s);

    let result = hasher.finalize();
    return hash_vector_to_string(result.to_vec());
}

fn create_hash_string512(s: &str) -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(s);

    let result = hasher.finalize();
    return hash_vector_to_string(result.to_vec());
}


fn main() {
    println!("SHA-3-224(\"\"):");
    let hashed = create_hash_string224("");
    println!("{hashed}");

    println!("SHA-3-256(\"\"):");
    let hashed = create_hash_string256("");
    println!("{hashed}");

    println!("SHA-3-384(\"\"):");
    let hashed = create_hash_string384("");
    println!("{hashed}");

    println!("SHA-3-512(\"\"):");
    let hashed = create_hash_string512("");
    println!("{hashed}");

}
