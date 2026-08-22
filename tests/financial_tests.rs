use carteira_inteligente::auth::password::{hash_password, verify_password};
use carteira_inteligente::models::PositionSummary;
use carteira_inteligente::services::RebalanceService;

#[test]
fn test_password_hash_and_verify() {
    let password = "MinhaSenhaSuperSegura123!";
    let hash = hash_password(password).expect("Erro ao gerar hash da senha");
    assert!(hash.starts_with("$argon2"));

    let is_valid = verify_password(password, &hash).expect("Erro ao verificar senha");
    assert!(is_valid);

    let is_invalid = verify_password("SenhaErrada", &hash).expect("Erro ao verificar senha");
    assert!(!is_invalid);
}

#[test]
fn test_smart_rebalance_allocation() {
    // Simula uma carteira com 2 ativos:
    // Ativo 1: Ações PETR4 (Meta: 50%, Atual: R$ 4.000 -> 80%)
    // Ativo 2: FII HGLG11 (Meta: 50%, Atual: R$ 1.000 -> 20%)
    // Novo aporte de R$ 3.000,00 -> O algoritmo deve destinar mais para HGLG11 para aproximar das metas
    let positions = vec![
        PositionSummary {
            asset_id: "1".to_string(),
            ticker: "PETR4".to_string(),
            name: "Petrobras".to_string(),
            asset_class: "Ações".to_string(),
            quantity: 100.0,
            average_price: 35.0,
            current_price: 40.0,
            total_invested: 3500.0,
            current_total_value: 4000.0,
            profit_loss: 500.0,
            profit_loss_percentage: 14.28,
            total_income: 150.0,
            current_weight_percentage: 80.0,
            target_percentage: 50.0,
        },
        PositionSummary {
            asset_id: "2".to_string(),
            ticker: "HGLG11".to_string(),
            name: "CSHG Logística".to_string(),
            asset_class: "FIIs".to_string(),
            quantity: 10.0,
            average_price: 100.0,
            current_price: 100.0,
            total_invested: 1000.0,
            current_total_value: 1000.0,
            profit_loss: 0.0,
            profit_loss_percentage: 0.0,
            total_income: 80.0,
            current_weight_percentage: 20.0,
            target_percentage: 50.0,
        },
    ];

    let aporte = 3000.0;
    let result = RebalanceService::calculate_rebalance(&positions, aporte);

    assert_eq!(result.total_portfolio_before, 5000.0);
    assert_eq!(result.total_portfolio_after, 8000.0);

    // Meta total é 8000 * 50% = 4000 para cada.
    // PETR4 já tem 4000 -> Déficit é 0.
    // HGLG11 tem 1000 -> Déficit é 3000.
    // Todo o aporte de 3000 deve ser recomendado para HGLG11!
    let hglg_rec = result.recommendations.iter().find(|r| r.ticker == "HGLG11").unwrap();
    let petr_rec = result.recommendations.iter().find(|r| r.ticker == "PETR4").unwrap();

    assert!(hglg_rec.recommended_buy_amount > 2900.0);
    assert_eq!(petr_rec.recommended_buy_amount, 0.0);
}
