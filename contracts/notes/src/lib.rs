#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan informasi tagihan (Split Bill)
#[contracttype]
#[derive(Clone, Debug)]
pub struct Bill {
    pub id: u64,
    pub description: String,
    pub total_amount: u64,
    pub participants_count: u32,
    pub amount_per_person: u64,
}

// Storage key untuk data bill (maksimal 9 karakter untuk symbol_short)
const BILL_DATA: Symbol = symbol_short!("BILLS");

#[contract]
pub struct SplitBillContract;

#[contractimpl]
impl SplitBillContract {
    // Fungsi untuk melihat semua tagihan
    pub fn get_bills(env: Env) -> Vec<Bill> {
        // 1. Ambil data bills dari storage
        return env.storage().instance().get(&BILL_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk membuat tagihan baru
    pub fn create_bill(
        env: Env, 
        description: String, 
        total_amount: u64, 
        participants_count: u32
    ) -> String {
        // Validasi agar jumlah orang tidak 0 (mencegah error pembagian)
        if participants_count == 0 {
            return String::from_str(&env, "Jumlah orang tidak boleh 0");
        }

        // 1. Ambil data bills dari storage
        let mut bills: Vec<Bill> = env.storage().instance().get(&BILL_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Hitung jumlah yang harus dibayar per orang
        let split_amount = total_amount / (participants_count as u64);

        // 3. Buat object bill baru
        let new_bill = Bill {
            id: env.prng().gen::<u64>(),
            description,
            total_amount,
            participants_count,
            amount_per_person: split_amount,
        };
        
        // 4. Tambahkan bill baru ke daftar bills
        bills.push_back(new_bill);
        
        // 5. Simpan kembali ke storage
        env.storage().instance().set(&BILL_DATA, &bills);
        
        return String::from_str(&env, "Tagihan berhasil ditambahkan");
    }

    // Fungsi untuk menghapus tagihan berdasarkan id
    pub fn delete_bill(env: Env, id: u64) -> String {
        // 1. Ambil data bills dari storage 
        let mut bills: Vec<Bill> = env.storage().instance().get(&BILL_DATA).unwrap_or(Vec::new(&env));

        // 2. Cari index bill yang akan dihapus
        for i in 0..bills.len() {
            if bills.get(i).unwrap().id == id {
                bills.remove(i);

                // 3. Simpan perubahan ke storage
                env.storage().instance().set(&BILL_DATA, &bills);
                return String::from_str(&env, "Berhasil menghapus tagihan");
            }
        }

        return String::from_str(&env, "Tagihan tidak ditemukan");
    }
}

mod test;