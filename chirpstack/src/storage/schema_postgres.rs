// @generated automatically by Diesel CLI.

diesel::table! {
    api_key (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        #[max_length = 100]
        name -> Varchar,
        is_admin -> Bool,
        tenant_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    application (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        mqtt_tls_cert -> Nullable<Bytea>,
        tags -> Jsonb,
    }
}

diesel::table! {
    application_integration (application_id, kind) {
        application_id -> Uuid,
        #[max_length = 20]
        kind -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        configuration -> Jsonb,
    }
}

diesel::table! {
    device (dev_eui) {
        dev_eui -> Bytea,
        application_id -> Uuid,
        device_profile_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_seen_at -> Nullable<Timestamptz>,
        scheduler_run_after -> Nullable<Timestamptz>,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        external_power_source -> Bool,
        battery_level -> Nullable<Numeric>,
        margin -> Nullable<Int4>,
        dr -> Nullable<Int2>,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        altitude -> Nullable<Float4>,
        dev_addr -> Nullable<Bytea>,
        #[max_length = 1]
        enabled_class -> Bpchar,
        skip_fcnt_check -> Bool,
        is_disabled -> Bool,
        tags -> Jsonb,
        variables -> Jsonb,
        join_eui -> Bytea,
        secondary_dev_addr -> Nullable<Bytea>,
        device_session -> Nullable<Bytea>,
        app_layer_params -> Jsonb,
    }
}

diesel::table! {
    device_keys (dev_eui) {
        dev_eui -> Bytea,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        nwk_key -> Bytea,
        app_key -> Bytea,
        dev_nonces -> Jsonb,
        join_nonce -> Int4,
        gen_app_key -> Bytea,
    }
}

diesel::table! {
    device_profile (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 10]
        region -> Varchar,
        #[max_length = 10]
        mac_version -> Varchar,
        #[max_length = 20]
        reg_params_revision -> Varchar,
        #[max_length = 100]
        adr_algorithm_id -> Varchar,
        #[max_length = 20]
        payload_codec_runtime -> Varchar,
        uplink_interval -> Int4,
        device_status_req_interval -> Int4,
        supports_otaa -> Bool,
        supports_class_b -> Bool,
        supports_class_c -> Bool,
        tags -> Jsonb,
        payload_codec_script -> Text,
        flush_queue_on_activate -> Bool,
        description -> Text,
        measurements -> Jsonb,
        auto_detect_measurements -> Bool,
        #[max_length = 100]
        region_config_id -> Nullable<Varchar>,
        allow_roaming -> Bool,
        rx1_delay -> Int2,
        abp_params -> Nullable<Jsonb>,
        class_b_params -> Nullable<Jsonb>,
        class_c_params -> Nullable<Jsonb>,
        relay_params -> Nullable<Jsonb>,
        app_layer_params -> Jsonb,
    }
}

diesel::table! {
    device_profile_template (id) {
        id -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        #[max_length = 100]
        vendor -> Varchar,
        #[max_length = 100]
        firmware -> Varchar,
        #[max_length = 10]
        region -> Varchar,
        #[max_length = 10]
        mac_version -> Varchar,
        #[max_length = 20]
        reg_params_revision -> Varchar,
        #[max_length = 100]
        adr_algorithm_id -> Varchar,
        #[max_length = 20]
        payload_codec_runtime -> Varchar,
        payload_codec_script -> Text,
        uplink_interval -> Int4,
        device_status_req_interval -> Int4,
        flush_queue_on_activate -> Bool,
        supports_otaa -> Bool,
        supports_class_b -> Bool,
        supports_class_c -> Bool,
        class_b_timeout -> Int4,
        class_b_ping_slot_periodicity -> Int4,
        class_b_ping_slot_dr -> Int2,
        class_b_ping_slot_freq -> Int8,
        class_c_timeout -> Int4,
        abp_rx1_delay -> Int2,
        abp_rx1_dr_offset -> Int2,
        abp_rx2_dr -> Int2,
        abp_rx2_freq -> Int8,
        tags -> Jsonb,
        measurements -> Jsonb,
        auto_detect_measurements -> Bool,
    }
}

diesel::table! {
    device_queue_item (id) {
        id -> Uuid,
        dev_eui -> Bytea,
        created_at -> Timestamptz,
        f_port -> Int2,
        confirmed -> Bool,
        data -> Bytea,
        is_pending -> Bool,
        f_cnt_down -> Nullable<Int8>,
        timeout_after -> Nullable<Timestamptz>,
        is_encrypted -> Bool,
        expires_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    fuota_deployment (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        #[max_length = 100]
        name -> Varchar,
        application_id -> Uuid,
        device_profile_id -> Uuid,
        multicast_addr -> Bytea,
        multicast_key -> Bytea,
        #[max_length = 1]
        multicast_group_type -> Bpchar,
        #[max_length = 20]
        multicast_class_c_scheduling_type -> Varchar,
        multicast_dr -> Int2,
        multicast_class_b_ping_slot_periodicity -> Int2,
        multicast_frequency -> Int8,
        multicast_timeout -> Int2,
        multicast_session_start -> Nullable<Timestamptz>,
        multicast_session_end -> Nullable<Timestamptz>,
        unicast_max_retry_count -> Int2,
        fragmentation_fragment_size -> Int2,
        fragmentation_redundancy_percentage -> Int2,
        fragmentation_session_index -> Int2,
        fragmentation_matrix -> Int2,
        fragmentation_block_ack_delay -> Int2,
        fragmentation_descriptor -> Bytea,
        #[max_length = 20]
        request_fragmentation_session_status -> Varchar,
        payload -> Bytea,
        on_complete_set_device_tags -> Jsonb,
    }
}

diesel::table! {
    fuota_deployment_device (fuota_deployment_id, dev_eui) {
        fuota_deployment_id -> Uuid,
        dev_eui -> Bytea,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
        mc_group_setup_completed_at -> Nullable<Timestamptz>,
        mc_session_completed_at -> Nullable<Timestamptz>,
        frag_session_setup_completed_at -> Nullable<Timestamptz>,
        frag_status_completed_at -> Nullable<Timestamptz>,
        error_msg -> Text,
    }
}

diesel::table! {
    fuota_deployment_gateway (fuota_deployment_id, gateway_id) {
        fuota_deployment_id -> Uuid,
        gateway_id -> Bytea,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    fuota_deployment_job (fuota_deployment_id, job) {
        fuota_deployment_id -> Uuid,
        #[max_length = 20]
        job -> Varchar,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
        max_retry_count -> Int2,
        attempt_count -> Int2,
        scheduler_run_after -> Timestamptz,
        warning_msg -> Text,
        error_msg -> Text,
    }
}

diesel::table! {
    gateway (gateway_id) {
        gateway_id -> Bytea,
        tenant_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_seen_at -> Nullable<Timestamptz>,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        latitude -> Float8,
        longitude -> Float8,
        altitude -> Float4,
        stats_interval_secs -> Int4,
        tls_certificate -> Nullable<Bytea>,
        tags -> Jsonb,
        properties -> Jsonb,
    }
}

diesel::table! {
    multicast_group (id) {
        id -> Uuid,
        application_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 10]
        region -> Varchar,
        mc_addr -> Bytea,
        mc_nwk_s_key -> Bytea,
        mc_app_s_key -> Bytea,
        f_cnt -> Int8,
        #[max_length = 1]
        group_type -> Bpchar,
        dr -> Int2,
        frequency -> Int8,
        class_b_ping_slot_periodicity -> Int2,
        #[max_length = 20]
        class_c_scheduling_type -> Varchar,
    }
}

diesel::table! {
    multicast_group_device (multicast_group_id, dev_eui) {
        multicast_group_id -> Uuid,
        dev_eui -> Bytea,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    multicast_group_gateway (multicast_group_id, gateway_id) {
        multicast_group_id -> Uuid,
        gateway_id -> Bytea,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    multicast_group_queue_item (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        scheduler_run_after -> Timestamptz,
        multicast_group_id -> Uuid,
        gateway_id -> Bytea,
        f_cnt -> Int8,
        f_port -> Int2,
        data -> Bytea,
        emit_at_time_since_gps_epoch -> Nullable<Int8>,
        expires_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    relay_device (relay_dev_eui, dev_eui) {
        relay_dev_eui -> Bytea,
        dev_eui -> Bytea,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    relay_gateway (tenant_id, relay_id) {
        tenant_id -> Uuid,
        relay_id -> Bytea,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_seen_at -> Nullable<Timestamptz>,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        stats_interval_secs -> Int4,
        #[max_length = 100]
        region_config_id -> Varchar,
    }
}

diesel::table! {
    tenant (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        can_have_gateways -> Bool,
        max_device_count -> Int4,
        max_gateway_count -> Int4,
        private_gateways_up -> Bool,
        private_gateways_down -> Bool,
        tags -> Jsonb,
    }
}

diesel::table! {
    tenant_user (tenant_id, user_id) {
        tenant_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_admin -> Bool,
        is_device_admin -> Bool,
        is_gateway_admin -> Bool,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        external_id -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_admin -> Bool,
        is_active -> Bool,
        email -> Text,
        email_verified -> Bool,
        #[max_length = 200]
        password_hash -> Varchar,
        note -> Text,
    }
}

diesel::joinable!(api_key -> tenant (tenant_id));
diesel::joinable!(application -> tenant (tenant_id));
diesel::joinable!(application_integration -> application (application_id));
diesel::joinable!(device -> application (application_id));
diesel::joinable!(device -> device_profile (device_profile_id));
diesel::joinable!(device_keys -> device (dev_eui));
diesel::joinable!(device_profile -> tenant (tenant_id));
diesel::joinable!(device_queue_item -> device (dev_eui));
diesel::joinable!(fuota_deployment -> application (application_id));
diesel::joinable!(fuota_deployment -> device_profile (device_profile_id));
diesel::joinable!(fuota_deployment_device -> device (dev_eui));
diesel::joinable!(fuota_deployment_device -> fuota_deployment (fuota_deployment_id));
diesel::joinable!(fuota_deployment_gateway -> fuota_deployment (fuota_deployment_id));
diesel::joinable!(fuota_deployment_gateway -> gateway (gateway_id));
diesel::joinable!(fuota_deployment_job -> fuota_deployment (fuota_deployment_id));
diesel::joinable!(gateway -> tenant (tenant_id));
diesel::joinable!(multicast_group -> application (application_id));
diesel::joinable!(multicast_group_device -> device (dev_eui));
diesel::joinable!(multicast_group_device -> multicast_group (multicast_group_id));
diesel::joinable!(multicast_group_gateway -> gateway (gateway_id));
diesel::joinable!(multicast_group_gateway -> multicast_group (multicast_group_id));
diesel::joinable!(multicast_group_queue_item -> gateway (gateway_id));
diesel::joinable!(multicast_group_queue_item -> multicast_group (multicast_group_id));
diesel::joinable!(relay_gateway -> tenant (tenant_id));
diesel::joinable!(tenant_user -> tenant (tenant_id));
diesel::joinable!(tenant_user -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_key,
    application,
    application_integration,
    device,
    device_keys,
    device_profile,
    device_profile_template,
    device_queue_item,
    fuota_deployment,
    fuota_deployment_device,
    fuota_deployment_gateway,
    fuota_deployment_job,
    gateway,
    multicast_group,
    multicast_group_device,
    multicast_group_gateway,
    multicast_group_queue_item,
    relay_device,
    relay_gateway,
    tenant,
    tenant_user,
    user,
);
