# Lumenza
Lumenza is an photo manager based off of [libospl](https://github.com/libospl/libospl).

This crate is the half of two main parts: it manages all the photos and does all the background things like importing. All the info is stored in the database. This part is cross-platform. 

The user interface, communicating with this crate, is the other half. This permits photo libraries to be synced independently because the photo management is the same across all possible platforms. Additional crates can provide geolocation or AI-based tagging.

## Documentation
To view documentation, simply run `cargo doc --no-deps`, and open the HTML file in the target/doc directory. 
