#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk menyimpan informasi peminjaman ruangan
#[contracttype]
#[derive(Clone, Debug)]
pub struct Booking {
    pub id: u64,
    pub room_name: String,
    pub booker_name: String,
    pub time_slot: String, // Contoh: "Senin, 08:00-10:00"
}

// Storage key untuk data peminjaman (maksimal 9 karakter)
const BOOKING_DATA: Symbol = symbol_short!("BOOKINGS");

#[contract]
pub struct RoomBookingContract;

#[contractimpl]
impl RoomBookingContract {
    // Fungsi untuk melihat semua jadwal peminjaman ruangan
    pub fn get_bookings(env: Env) -> Vec<Booking> {
        return env.storage().instance().get(&BOOKING_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk meminjam ruangan baru
    pub fn create_booking(
        env: Env, 
        room_name: String, 
        booker_name: String, 
        time_slot: String
    ) -> String {
        let mut bookings: Vec<Booking> = env.storage().instance().get(&BOOKING_DATA).unwrap_or(Vec::new(&env));
        
        // Validasi: Cek apakah ruangan sudah dipesan di waktu yang sama
        for i in 0..bookings.len() {
            let existing_booking = bookings.get(i).unwrap();
            
            // Jika nama ruangan dan waktu sama, tolak peminjaman
            if existing_booking.room_name == room_name && existing_booking.time_slot == time_slot {
                return String::from_str(&env, "Gagal: Ruangan sudah dipesan pada waktu tersebut");
            }
        }

        // Jika kosong, buat object booking baru
        let new_booking = Booking {
            id: env.prng().gen::<u64>(),
            room_name,
            booker_name,
            time_slot,
        };
        
        // Tambahkan jadwal baru ke daftar
        bookings.push_back(new_booking);
        
        // Simpan ke storage
        env.storage().instance().set(&BOOKING_DATA, &bookings);
        
        return String::from_str(&env, "Berhasil meminjam ruangan");
    }

    // Fungsi untuk membatalkan (menghapus) peminjaman berdasarkan ID
    pub fn cancel_booking(env: Env, id: u64) -> String {
        let mut bookings: Vec<Booking> = env.storage().instance().get(&BOOKING_DATA).unwrap_or(Vec::new(&env));

        for i in 0..bookings.len() {
            if bookings.get(i).unwrap().id == id {
                bookings.remove(i);
                env.storage().instance().set(&BOOKING_DATA, &bookings);
                return String::from_str(&env, "Berhasil membatalkan pesanan ruangan");
            }
        }

        return String::from_str(&env, "Pesanan tidak ditemukan");
    }
}

mod test;