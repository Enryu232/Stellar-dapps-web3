Stellar Room Booking DApp
Stellar Room Booking DApp - Blockchain-Based Decentralized Room Scheduling System

Project Description
Stellar Room Booking DApp is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, transparent, and collision-proof platform for scheduling and managing laboratory and meeting room reservations. By leveraging blockchain technology, the application ensures that booking records are public, immutable, and strictly validated to prevent double-booking.

The system allows users to reserve a room by specifying the room name, their name, and a specific time slot. The smart contract automatically verifies availability before confirming the reservation, making it ideal for university laboratories, coworking spaces, and public facilities.

Project Vision
Our vision is to modernize facility management and scheduling by:

Decentralizing Schedules: Moving booking databases from centralized servers to a highly accessible distributed ledger.

Preventing Conflicts: Automating double-booking prevention at the contract level to ensure fairness and accuracy.

Guaranteeing Transparency: Providing a public, tamper-proof record of facility usage.

Empowering Users: Allowing students and professionals to reserve spaces without relying on slow administrative bureaucracy.

Key Features
1. Collision-Proof Booking
Create a room reservation with a single function call.

The contract automatically checks existing schedules and rejects attempts to book the same room at the same time.

Automated ID generation for unique reservation tracking.

2. Transparent Schedule Retrieval
Fetch all current reservations in a single call.

Structured data representation containing room names, booker names, and time slots.

Real-time schedule synchronization with the blockchain state.

3. Secure Cancellation
Cancel specific reservations using their unique IDs.

Immediate availability update for other users once a booking is canceled.

4. Stellar Network Integration
Leverages the high speed and incredibly low cost of the Stellar network.

Built using the modern, Rust-based Soroban Smart Contract SDK.

Contract Details
Contract Address: [YOUR_CONTRACT_ADDRESS_HERE]

Future Scope
Short-Term Enhancements
Time Slot Standardization: Implement strict formatting for time slots (e.g., standardizing date and hour inputs).

Access Control: Ensure only the person who created the booking (via their wallet address) can cancel it.

Department Categories: Add tags to differentiate between IT Labs, Meeting Rooms, and Auditoriums.

Medium-Term Development
Deposit System: Require a small token deposit to book a room, which is refunded upon checkout, to prevent spam bookings.

QR Code Integration: Generate a QR code from the booking ID for room entry verification.

Frontend Calendar UI: Build a visual calendar interface to make selecting available time slots more intuitive.

Technical Requirements
Soroban SDK

Rust programming language

Stellar blockchain network

Getting Started
Deploy the smart contract to Stellar's Soroban network and interact with it using the three main functions:

create_booking(room_name: String, booker_name: String, time_slot: String) - Request a room reservation.

get_bookings() - Retrieve the entire schedule of booked rooms.

cancel_booking(id: u64) - Cancel and remove a specific reservation.