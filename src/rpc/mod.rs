/**********************************************************\
|                                                          |
|                          hprose                          |
|                                                          |
| Official WebSite: http://www.hprose.com/                 |
|                   http://www.hprose.org/                 |
|                                                          |
\**********************************************************/
/**********************************************************\
 *                                                        *
 * rpc/mod.rs                                             *
 *                                                        *
 * hprose rpc module for Rust.                            *
 *                                                        *
 * LastModified: Sep 22, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

mod client;
mod http_client;
mod result_mode;

pub use self::client::{InvokeOptions, InvokeResult, InvokeError, Client, Transporter, ClientContext, BaseClient};
pub use self::http_client::HttpClient;
pub use self::result_mode::ResultMode;
