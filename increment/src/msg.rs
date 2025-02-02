use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {% raw %}{{% endraw %}{% unless version == "minimal" %}
    pub count: i32,
{% endunless %}}

#[cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
pub enum ExecuteMsg {% raw %}{{% endraw %}{% unless version == "minimal" %}
    Increment {},
    Reset { count: i32 },
{% endunless %}}

#[cw_serde]
#[derive(QueryResponses)]
#[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))]
pub enum QueryMsg {% raw %}{{% endraw %}{% unless version == "minimal" %}
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
{% endunless %}}
{% unless version == "minimal" %}
// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
{% endunless %}