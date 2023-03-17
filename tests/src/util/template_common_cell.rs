use das_types_std::constants::AccountStatus;
use serde_json::{json, Value};

use super::accounts::*;
use super::constants::*;
use super::template_generator::*;
use super::util;

pub fn push_input_apply_register_cell(template: &mut TemplateGenerator, cell_partial: Value, since: Option<u64>) {
    let mut cell = json!({
        "header": {
            "height": HEIGHT - 1,
            "timestamp": TIMESTAMP - DAY_SEC,
        },
        "lock": {
            "code_hash": "{{fake-secp256k1-blake160-signhash-all}}",
            "args": OWNER_WITHOUT_TYPE
        },
        "type": {
            "code_hash": "{{apply-register-cell-type}}"
        },
        "data": {
            "account": ACCOUNT_1,
            "height": Value::Null,
            "timestamp": Value::Null,
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_input(cell, since, None);
}

pub fn push_output_apply_register_cell(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "lock": {
            "code_hash": "{{fake-secp256k1-blake160-signhash-all}}",
            "args": OWNER_WITHOUT_TYPE
        },
        "type": {
            "code_hash": "{{apply-register-cell-type}}"
        },
        "data": {
            "account": ACCOUNT_1,
            "height": Value::Null,
            "timestamp": Value::Null,
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_output(cell, None);
}

pub fn push_dep_pre_account_cell(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "capacity": util::gen_account_cell_capacity(5),
        "lock": {
            "code_hash": "{{always_success}}"
        },
        "type": {
            "code_hash": "{{pre-account-cell-type}}"
        },
        "witness": {
            "account": ACCOUNT_1,
            "refund_lock": {
                "code_hash": "{{fake-secp256k1-blake160-signhash-all}}",
                "args": "0x0000000000000000000000000000000000001111"
            },
            "owner_lock_args": "0x050000000000000000000000000000000000001111050000000000000000000000000000000000001111",
            "inviter_id": Value::Null,
            "inviter_lock": Value::Null,
            "channel_lock": Value::Null,
            "price": {
                "length": 5,
                "new": ACCOUNT_PRICE_5_CHAR,
                "renew": ACCOUNT_PRICE_5_CHAR
            },
            "quote": CKB_QUOTE,
            "invited_discount": INVITED_DISCOUNT,
            "created_at": Value::Null
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_dep(cell, None);
}

pub fn push_input_pre_account_cell(template: &mut TemplateGenerator, cell_partial: Value, since: Option<u64>) {
    let mut cell = json!({
        "capacity": util::gen_register_fee(5, false),
        "lock": {
            "code_hash": "{{always_success}}"
        },
        "type": {
            "code_hash": "{{pre-account-cell-type}}"
        },
        "witness": {
            "account": ACCOUNT_1,
            "refund_lock": {
                "code_hash": "{{fake-secp256k1-blake160-signhash-all}}",
                "args": OWNER_WITHOUT_TYPE
            },
            "owner_lock_args": gen_das_lock_args(OWNER, None),
            "inviter_id": INVITER_ID,
            "inviter_lock": Value::Null,
            "channel_lock": Value::Null,
            "price": {
                "length": 5,
                "new": ACCOUNT_PRICE_5_CHAR,
                "renew": ACCOUNT_PRICE_5_CHAR
            },
            "quote": CKB_QUOTE,
            "invited_discount": INVITED_DISCOUNT,
            "created_at": Value::Null,
            "initial_records": [],
            "initial_cross_chain": {
                "coin_type": 0,
                "chain_id": 0,
                "checked": 0
            }
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_input(cell, since, Some(3));
}

pub fn push_output_pre_account_cell_v1(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "capacity": util::gen_register_fee(5, false),
        "lock": {
            "code_hash": "{{always_success}}"
        },
        "type": {
            "code_hash": "{{pre-account-cell-type}}"
        },
        "witness": {
            "account": ACCOUNT_1,
            "refund_lock": {
                "code_hash": "{{fake-secp256k1-blake160-signhash-all}}",
                "args": OWNER_WITHOUT_TYPE
            },
            "owner_lock_args": gen_das_lock_args(OWNER, None),
            "inviter_id": Value::Null,
            "inviter_lock": Value::Null,
            "channel_lock": Value::Null,
            "price": {
                "length": 5,
                "new": ACCOUNT_PRICE_5_CHAR,
                "renew": ACCOUNT_PRICE_5_CHAR
            },
            "quote": CKB_QUOTE,
            "invited_discount": 0,
            "created_at": TIMESTAMP
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_output(cell, Some(1));
}

pub fn push_output_pre_account_cell_v2(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "capacity": util::gen_register_fee(5, false),
        "lock": {
            "code_hash": "{{always_success}}"
        },
        "type": {
            "code_hash": "{{pre-account-cell-type}}"
        },
        "witness": {
            "account": ACCOUNT_1,
            "refund_lock": {
                "code_hash": "{{fake-secp256k1-blake160-signhash-all}}",
                "args": OWNER_WITHOUT_TYPE
            },
            "owner_lock_args": gen_das_lock_args(OWNER, None),
            "inviter_id": Value::Null,
            "inviter_lock": Value::Null,
            "channel_lock": Value::Null,
            "price": {
                "length": 5,
                "new": ACCOUNT_PRICE_5_CHAR,
                "renew": ACCOUNT_PRICE_5_CHAR
            },
            "quote": CKB_QUOTE,
            "invited_discount": 0,
            "created_at": TIMESTAMP,
            "initial_records": [
                {
                    "type": "address",
                    "key": "60",
                    "label": "Personal",
                    "value": OWNER_WITHOUT_TYPE,
                }
            ],
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_output(cell, Some(2));
}

pub fn push_output_pre_account_cell(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "capacity": util::gen_register_fee(5, false),
        "lock": {
            "code_hash": "{{always_success}}"
        },
        "type": {
            "code_hash": "{{pre-account-cell-type}}"
        },
        "witness": {
            "account": ACCOUNT_1,
            "refund_lock": {
                "code_hash": "{{fake-secp256k1-blake160-signhash-all}}",
                "args": OWNER_WITHOUT_TYPE
            },
            "owner_lock_args": gen_das_lock_args(OWNER, None),
            "inviter_id": Value::Null,
            "inviter_lock": Value::Null,
            "channel_lock": Value::Null,
            "price": {
                "length": 5,
                "new": ACCOUNT_PRICE_5_CHAR,
                "renew": ACCOUNT_PRICE_5_CHAR
            },
            "quote": CKB_QUOTE,
            "invited_discount": 0,
            "created_at": TIMESTAMP,
            "initial_records": [
                {
                    "type": "address",
                    "key": "60",
                    "label": "Personal",
                    "value": OWNER_WITHOUT_TYPE,
                }
            ],
            "initial_cross_chain": {
                "coin_type": 0,
                "chain_id": 0,
                "checked": 0
            }
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_output(cell, Some(3));
}

pub fn push_dep_account_cell(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "capacity": util::gen_account_cell_capacity(5),
        "lock": {
            "owner_lock_args": OWNER,
            "manager_lock_args": MANAGER
        },
        "type": {
            "code_hash": "{{account-cell-type}}"
        },
        "data": {
            "account": ACCOUNT_1,
            "next": "yyyyy.bit",
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "witness": {
            "account": ACCOUNT_1,
            "registered_at": 0,
            "last_transfer_account_at": 0,
            "last_edit_manager_at": 0,
            "last_edit_records_at": 0,
            "status": (AccountStatus::Normal as u8)
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_dep(cell, Some(3));
}

pub fn push_input_account_cell(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "capacity": util::gen_account_cell_capacity(5),
        "lock": {
            "owner_lock_args": OWNER,
            "manager_lock_args": MANAGER
        },
        "type": {
            "code_hash": "{{account-cell-type}}"
        },
        "data": {
            "account": ACCOUNT_1,
            "next": "yyyyy.bit",
            "expired_at": u64::MAX,
        },
        "witness": {
            "account": ACCOUNT_1,
            "registered_at": 0,
            "last_transfer_account_at": 0,
            "last_edit_manager_at": 0,
            "last_edit_records_at": 0,
            "status": (AccountStatus::Normal as u8),
            "enable_sub_account": 0,
            "renew_sub_account_price": 0,
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_input(cell, None, Some(3));
    template.push_das_lock_witness("0000000000000000000000000000000000000000000000000000000000000000");
}

pub fn push_output_account_cell(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "capacity": util::gen_account_cell_capacity(5),
        "lock": {
            "owner_lock_args": OWNER,
            "manager_lock_args": MANAGER
        },
        "type": {
            "code_hash": "{{account-cell-type}}"
        },
        "data": {
            "account": ACCOUNT_1,
            "next": "yyyyy.bit",
            "expired_at": u64::MAX,
        },
        "witness": {
            "account": ACCOUNT_1,
            "registered_at": 0,
            "last_transfer_account_at": 0,
            "last_edit_manager_at": 0,
            "last_edit_records_at": 0,
            "status": (AccountStatus::Normal as u8),
            "enable_sub_account": 0,
            "renew_sub_account_price": 0,
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_output(cell, Some(3));
}

pub fn push_input_account_cell_v2(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "capacity": util::gen_account_cell_capacity(5),
        "lock": {
            "owner_lock_args": OWNER,
            "manager_lock_args": MANAGER
        },
        "type": {
            "code_hash": "{{account-cell-type}}"
        },
        "data": {
            "account": ACCOUNT_1,
            "next": "yyyyy.bit",
            "expired_at": u64::MAX,
        },
        "witness": {
            "account": ACCOUNT_1,
            "registered_at": 0,
            "status": (AccountStatus::Normal as u8)
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_input(cell, None, Some(2));
    template.push_das_lock_witness("0000000000000000000000000000000000000000000000000000000000000000");
}

pub fn push_input_sub_account_cell(template: &mut TemplateGenerator, cell_partial: Value) {
    push_input_sub_account_cell_v2(template, cell_partial, ACCOUNT_1);
}

pub fn push_input_sub_account_cell_v2(template: &mut TemplateGenerator, cell_partial: Value, account: &str) {
    let das_profit = match cell_partial["data"]["das_profit"].as_u64() {
        Some(val) => val,
        _ => 0,
    };
    let owner_profit = match cell_partial["data"]["owner_profit"].as_u64() {
        Some(val) => val,
        _ => 0,
    };

    let mut cell = json!({
        "capacity": SUB_ACCOUNT_BASIC_CAPACITY + SUB_ACCOUNT_PREPARED_FEE_CAPACITY + das_profit + owner_profit,
        "lock": {
            "code_hash": "{{always_success}}"
        },
        "type": {
            "code_hash": "{{sub-account-cell-type}}",
            "args": account
        },
        "data": {
            "root": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "das_profit": 0,
            "owner_profit": 0,
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_input(cell, None, None);
}

pub fn push_output_sub_account_cell(template: &mut TemplateGenerator, cell_partial: Value) {
    push_output_sub_account_cell_v2(template, cell_partial, ACCOUNT_1);
}

pub fn push_output_sub_account_cell_v2(template: &mut TemplateGenerator, cell_partial: Value, account: &str) {
    let das_profit = match cell_partial["data"]["das_profit"].as_u64() {
        Some(val) => val,
        _ => 0,
    };
    let owner_profit = match cell_partial["data"]["owner_profit"].as_u64() {
        Some(val) => val,
        _ => 0,
    };

    let mut cell = json!({
        "capacity": SUB_ACCOUNT_BASIC_CAPACITY + SUB_ACCOUNT_PREPARED_FEE_CAPACITY + das_profit + owner_profit,
        "lock": {
            "code_hash": "{{always_success}}"
        },
        "type": {
            "code_hash": "{{sub-account-cell-type}}",
            "args": account
        },
        "data": {
            "root": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "das_profit": 0,
            "owner_profit": 0,
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_output(cell, None);
}

pub fn push_input_income_cell(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "lock": {
            "code_hash": "{{always_success}}"
        },
        "type": {
            "code_hash": "{{income-cell-type}}"
        },
        "witness": {
            "creator": {
                "code_hash": "{{fake-secp256k1-blake160-signhash-all}}",
                "args": COMMON_INCOME_CREATOR
            },
            "records": []
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_input(cell, None, None);
    template.push_empty_witness();
}

pub fn push_input_income_cell_no_creator(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "lock": {
            "code_hash": "{{always_success}}"
        },
        "type": {
            "code_hash": "{{income-cell-type}}"
        },
        "witness": {
            "records": []
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_input(cell, None, None);
    template.push_empty_witness();
}

pub fn push_output_income_cell(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "lock": {
            "code_hash": "{{always_success}}"
        },
        "type": {
            "code_hash": "{{income-cell-type}}"
        },
        "witness": {
            "creator": {
                "code_hash": "{{fake-secp256k1-blake160-signhash-all}}",
                "args": COMMON_INCOME_CREATOR
            },
            "records": []
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_output(cell, None);
}

pub fn push_output_income_cell_no_creator(template: &mut TemplateGenerator, cell_partial: Value) {
    let mut cell = json!({
        "lock": {
            "code_hash": "{{always_success}}"
        },
        "type": {
            "code_hash": "{{income-cell-type}}"
        },
        "witness": {
            "records": []
        }
    });
    util::merge_json(&mut cell, cell_partial);

    template.push_output(cell, None);
}

pub fn push_input_balance_cell(template: &mut TemplateGenerator, capacity: u64, owner: &str) {
    template.push_input(
        json!({
            "capacity": capacity.to_string(),
            "lock": {
                "owner_lock_args": owner,
                "manager_lock_args": owner,
            },
            "type": {
                "code_hash": "{{balance-cell-type}}"
            }
        }),
        None,
        None,
    );
    template.push_das_lock_witness("0000000000000000000000000000000000000000000000000000000000000000");
}

pub fn push_output_balance_cell(template: &mut TemplateGenerator, capacity: u64, owner: &str) {
    template.push_output(
        json!({
            "capacity": capacity.to_string(),
            "lock": {
                "owner_lock_args": owner,
                "manager_lock_args": owner,
            },
            "type": {
                "code_hash": "{{balance-cell-type}}"
            }
        }),
        None,
    );
}

pub fn push_output_balance_cell_without_type(template: &mut TemplateGenerator, capacity: u64, owner: &str) {
    template.push_output(
        json!({
            "capacity": capacity.to_string(),
            "lock": {
                "owner_lock_args": owner,
                "manager_lock_args": owner,
            }
        }),
        None,
    );
}

pub fn push_input_normal_cell(template: &mut TemplateGenerator, capacity: u64, args: &str) {
    template.push_input(
        json!({
            "capacity": capacity.to_string(),
            "lock": {
                "code_hash": "{{fake-secp256k1-blake160-signhash-all}}",
                "args": args
            }
        }),
        None,
        None,
    );
    template.push_empty_witness();
}

pub fn push_output_normal_cell(template: &mut TemplateGenerator, capacity: u64, args: &str) {
    template.push_output(
        json!({
            "capacity": capacity.to_string(),
            "lock": {
                "code_hash": "{{fake-secp256k1-blake160-signhash-all}}",
                "args": args
            }
        }),
        None,
    );
}

pub fn push_input_test_env_cell(template: &mut TemplateGenerator) {
    template.push_input(
        json!({
            "capacity": 0,
            "lock": {
                "code_hash": "{{always_success}}"
            },
            "type": {
                "code_hash": "{{test-env}}}"
            }
        }),
        None,
        None,
    );
    template.push_empty_witness();
}

pub fn push_input_playground_cell(template: &mut TemplateGenerator) {
    template.push_input(
        json!({
            "capacity": 0,
            "lock": {
                "code_hash": "{{always-success}}"
            },
            "type": {
                "code_hash": "{{playground}}"
            }
        }),
        None,
        None,
    );
    template.push_empty_witness();
}
