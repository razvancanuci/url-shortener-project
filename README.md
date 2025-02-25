
<span style="font-size: 20px;">**URL Shortener Project**</span>

<div style="display:flex; justify-content:space-between">
    <img src="assets/vue.png" alt="Vue" width = 100 height = 100 />
    <img src ="assets/quasar.png" alt="Quasar" width = 100 height = 100 />
    <img src ="assets/rust.png" alt="Rust" width = 100 height = 100 />
    <img src ="assets/postgres.png" alt="Postgres" width = 100 height = 100 />
    <img src ="assets/aws.png" alt="AWS" width = 100 height = 100 />
    <img src ="assets/azure-devops.png" alt="Azure Devops" width = 100 height = 100 />
</div>

<br />

A simple URL shortening service that allows users to convert long links into short, easy-to-share URLs.

## Features

- 🏷️ **Secure and Fast** – Instantly generates short URLs.
- 🔗 **Automatic Redirection** – The shortened URL redirects users to the original link.
- 🏞️ **QR Code Generation** – Generate a QR code for each shortened URL for easy sharing.

## Technologies Used

- **Backend:** Rust (Actix-web)
- **Database:** PostgreSQL (Amazon Aurora)
- **Frontend:** Vue (Quasar Framework)
- **Storage:** Amazon S3 with CloudFront for QR code image hosting


The project has the following diagram:

<img src="assets/diagram.png" alt="diagram">

<br />

Below there is a video with a demo run on local machine:

<video width="640" height="360" controls allowfullscreen>
  <source src="assets/demo.mp4" type="video/mp4">
</video>
