/*
 * Copyright (C) 2015 Benjamin Fry <benjaminfry@me.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! TCP protocol related components for DNS

mod tcp_client_connection;
mod tcp_client_stream;
pub mod tcp_stream;

pub use self::tcp_client_connection::TcpClientConnection;
pub use self::tcp_client_stream::TcpClientStream;
pub use self::tcp_stream::TcpStream;
