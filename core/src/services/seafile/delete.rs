// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use super::core::*;
use crate::raw::*;
use crate::*;
use std::sync::Arc;

pub struct SeafileDeleter {
    core: Arc<SeafileCore>,
}

impl SeafileDeleter {
    pub fn new(core: Arc<SeafileCore>) -> Self {
        Self { core }
    }
}

impl oio::OneShotDelete for SeafileDeleter {
    async fn delete_once(&self, path: String, _: OpDelete) -> Result<()> {
        self.core.delete(&path).await?;

        Ok(())
    }
}
