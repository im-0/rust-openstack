// Copyright 2017 Dmitry Tantsur <divius.inside@gmail.com>
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

//! Server management via Compute API.

use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};

use chrono::{DateTime, FixedOffset};

use super::super::{ApiResult, Sort};
use super::super::session::Session;
use super::super::utils::Query;
use super::v2::{V2API, protocol};


/// A query to server list.
#[derive(Clone, Debug)]
pub struct ServerQuery<'session> {
    session: &'session Session,
    query: Query,
}


/// Structure representing a summary of a single server.
#[derive(Clone, Debug)]
pub struct Server<'session> {
    session: &'session Session,
    inner: protocol::Server
}

/// Structure representing a summary of a single server.
#[derive(Clone, Debug)]
pub struct ServerSummary<'session> {
    session: &'session Session,
    inner: protocol::ServerSummary
}

/// List of servers.
pub type ServerList<'session> = Vec<ServerSummary<'session>>;

/// A reference to a flavor.
#[derive(Clone, Copy, Debug)]
pub struct FlavorRef<'session> {
    server: &'session Server<'session>
}

/// A reference to an image.
#[derive(Clone, Copy, Debug)]
pub struct ImageRef<'session> {
    server: &'session Server<'session>
}


impl<'session> Server<'session> {
    /// Load a Server object.
    pub(crate) fn new<Id: AsRef<str>>(session: &'session Session, id: Id)
            -> ApiResult<Server<'session>> {
        let inner = session.get_server(id)?;
        Ok(Server {
            session: session,
            inner: inner
        })
    }

    /// Get a reference to IPv4 address.
    pub fn access_ipv4(&self) -> &Option<Ipv4Addr> {
        &self.inner.accessIPv4
    }

    /// Get a reference to IPv6 address.
    pub fn access_ipv6(&self) -> &Option<Ipv6Addr> {
        &self.inner.accessIPv6
    }

    /// Get a reference to associated addresses.
    pub fn addresses(&self) -> &HashMap<String, Vec<protocol::ServerAddress>> {
        &self.inner.addresses
    }

    /// Get a reference to the availability zone.
    pub fn availability_zone(&self) -> &String {
        &self.inner.availability_zone
    }

    /// Get a reference to creation date and time.
    pub fn created_at(&self) -> &DateTime<FixedOffset> {
        &self.inner.created
    }

    /// Get a reference to the flavor.
    pub fn flavor(&'session self) -> FlavorRef<'session> {
        FlavorRef {
            server: self
        }
    }

    /// Get a reference to server unique ID.
    pub fn id(&self) -> &String {
        &self.inner.id
    }

    /// Get a reference to the image.
    pub fn image(&'session self) -> ImageRef<'session> {
        ImageRef {
            server: self
        }
    }

    /// Get a reference to server name.
    pub fn name(&self) -> &String {
        &self.inner.name
    }

    /// Get server status.
    pub fn status(&self) -> protocol::ServerStatus {
        self.inner.status
    }

    /// Get a reference to last update date and time.
    pub fn updated_at(&self) -> &DateTime<FixedOffset> {
        &self.inner.updated
    }
}

impl<'session> FlavorRef<'session> {
    /// Get a reference to flavor unique ID.
    pub fn id(&self) -> &'session String {
        &self.server.inner.flavor.id
    }

    // TODO: pub fn details(&self) -> ApiResult<Flavor>
}

impl<'session> ImageRef<'session> {
    /// Get a reference to image unique ID.
    pub fn id(&self) -> &'session String {
        &self.server.inner.image.id
    }

    // TODO: #[cfg(feature = "image")] pub fn details(&self) -> ApiResult<Image>
}

impl<'session> ServerSummary<'session> {
    /// Get a reference to server unique ID.
    pub fn id(&self) -> &String {
        &self.inner.id
    }

    /// Get a reference to server name.
    pub fn name(&self) -> &String {
        &self.inner.name
    }

    /// Get details.
    pub fn details(&self) -> ApiResult<Server<'session>> {
        Server::new(self.session, &self.inner.id)
    }
}

impl<'session> ServerQuery<'session> {
    pub(crate) fn new(session: &'session Session) -> ServerQuery<'session> {
        ServerQuery {
            session: session,
            query: Query::new(),
        }
    }

    /// Add marker to the request.
    pub fn with_marker<T: Into<String>>(mut self, marker: T) -> Self {
        self.query.push_str("marker", marker);
        self
    }

    /// Add limit to the request.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.query.push("limit", limit);
        self
    }

    /// Add sorting to the request.
    pub fn sort_by(mut self, sort: Sort<protocol::ServerSortKey>) -> Self {
        let (field, direction) = sort.into();
        self.query.push_str("sort_key", field);
        self.query.push("sort_dir", direction);
        self
    }

    /// Filter by IPv4 address that should be used to access the server.
    pub fn with_access_ip_v4(mut self, value: Ipv4Addr) -> Self {
        self.query.push("access_ip_v4", value);
        self
    }

    /// Filter by IPv6 address that should be used to access the server.
    pub fn with_access_ip_v6(mut self, value: Ipv6Addr) -> Self {
        self.query.push("access_ipv6", value);
        self
    }

    /// Filter by availability zone.
    pub fn with_availability_zone<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("availability_zone", value);
        self
    }

    /// Filter by flavor.
    pub fn with_flavor<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("flavor", value);
        self
    }

    /// Filter by host name.
    pub fn with_hostname<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("hostname", value);
        self
    }

    /// Filter by image ID.
    pub fn with_image<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("image", value);
        self
    }

    /// Filter by an IPv4 address.
    pub fn with_ip_v4(mut self, value: Ipv4Addr) -> Self {
        self.query.push("ip", value);
        self
    }

    /// Filter by an IPv6 address.
    pub fn with_ip_v6(mut self, value: Ipv6Addr) -> Self {
        self.query.push("ip6", value);
        self
    }

    /// Filter by server name (a database regular expression).
    pub fn with_name<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("name", value);
        self
    }

    /// Filter by power state.
    pub fn with_power_state<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("power_state", value);
        self
    }

    /// Filter by project ID (also commonly known as tenant ID).
    pub fn with_project_id<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("project_id", value);
        self
    }

    /// Filter by server status.
    pub fn with_status<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("status", value);
        self
    }

    /// Filter by user ID.
    pub fn with_user_id<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("user_id", value);
        self
    }

    /// Execute this request and return its result.
    #[allow(unused_results)]
    pub fn fetch(self) -> ApiResult<ServerList<'session>> {
        trace!("Listing compute servers with {:?}", self.query);
        let servers = self.session.list_servers(&self.query.0)?;
        debug!("Received {} compute servers", servers.len());
        trace!("Received servers: {:?}", servers);
        Ok(servers.into_iter().map(|x| ServerSummary {
            session: self.session,
            inner: x
        }).collect())
    }
}
