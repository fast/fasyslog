// Copyright 2024 FastLabs Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use fasyslog::Severity;

fn main() {
    let mut sender = fasyslog::sender::native_tls_well_known("127.0.0.1").unwrap();
    let mut generator = names::Generator::default();
    for _ in 0..100 {
        let name = generator.next().unwrap();
        let message = format!("Hello, {name}!");
        sender
            .send_rfc5424(Severity::ERROR, None::<String>, Vec::new(), message)
            .unwrap();
    }
    sender.flush().unwrap();
}
