# 💼 Carteira Inteligente de Investimentos (Fullstack em Rust)

Uma plataforma completa, robusta e moderna de **Gestão e Balanceamento Inteligente de Investimentos**, desenvolvida 100% em **Rust** utilizando **Axum**, **PostgreSQL** (via **SQLx**), **Askama** (templates HTML tipados em tempo de compilação) e autenticação segura com **Argon2id**.

---

## 🚀 Funcionalidades Principais

- 🔐 **Autenticação & Segurança de Alta Performance**:
  - Cadastro e Login com validação robusta.
  - Hashing de senhas com **Argon2id** (padrão ouro em criptografia).
  - Sessões gerenciadas via Cookies `HttpOnly` e `SameSite=Lax` com middleware no Axum (`AuthenticatedUser`).

- 📊 **Gestão Completa de Ativos**:
  - Suporte a múltiplas classes: **Ações**, **FIIs (Fundos Imobiliários)**, **Renda Fixa (Tesouro/CDB/LCI)**, **ETFs**, **Criptomoedas** e **Internacional**.
  - Acompanhamento de Cotação Atual, Preço Médio ponderado, Saldo Aplicado, Valor de Mercado e Metas percentuais.
  - Sincronização automática ou sob demanda de cotações de mercado.

- 💰 **Extrato de Transações & Proventos**:
  - Lançamento de **Compras**, **Vendas** e **Proventos (Dividendos, JCP, Rendimentos de FII)**.
  - Cálculo automático de **Preço Médio Ponderado**, resultado realizado e apuração de lucros/prejuízos.
  - Apuração de **Yield on Cost** e total de renda passiva recebida.

- 🪄 **Calculadora de Aporte Inteligente (Smart Rebalance)**:
  - O investidor informa quanto deseja investir no mês (ex: R$ 2.000,00) e o algoritmo calcula a distribuição ótima nos ativos que ficaram para trás da meta ("aportar no que ficou para trás"), sem necessidade de vender ativos para rebalancear a carteira.
  - Exportação/Cópia rápida de ordens com um clique.

- 🎨 **Interface Moderna & Responsiva**:
  - Design visual com **Glassmorphism**, suporte a **Dark Mode** e **Light Mode** persistido em `localStorage`.
  - Gráficos interativos com **Chart.js** (Alocação por Classe de Ativos em Rosca e Top Posições em Barras).
  - Templates Askama com verificação e tipagem estática durante a compilação do Rust.

---

## 🛠️ Tecnologias Utilizadas

| Componente | Tecnologia / Crate |
| :--- | :--- |
| **Linguagem** | Rust (Edição 2021) |
| **Framework Web** | Axum 0.7 & Tokio |
| **Banco de Dados** | PostgreSQL & SQLite fallback via SQLx 0.8 |
| **Template Engine** | Askama 0.12 (HTML Compile-time) & askama_axum |
| **Criptografia** | Argon2 0.5 & Tower Cookies |
| **Visual / Frontend** | CSS Vanilla Moderno, Glassmorphism, FontAwesome, Chart.js |

---

## ⚙️ Como Executar a Aplicação

### 1. Pré-requisitos
- [Rust & Cargo](https://rustup.rs/) (versão 1.75+)
- *(Opcional)* Docker & Docker Compose (para rodar PostgreSQL)

### 2. Configurar o Banco de Dados

Você pode rodar com **PostgreSQL** via Docker:
```bash
docker-compose up -d
```
Ou rodar diretamente com `cargo run` — caso o PostgreSQL não esteja ativo, o sistema iniciará automaticamente um banco de dados local SQLite (`carteira.db`) com todas as tabelas e migrações já aplicadas para desenvolvimento imediato.

### 3. Iniciar o Servidor
```bash
cargo run
```

Acesse no navegador:
👉 **[http://localhost:3000](http://localhost:3000)**

---

## 🧪 Testes Automatizados

Para executar os testes unitários e de integração de cálculo financeiro e segurança:
```bash
cargo test
```

---

## 📁 Estrutura do Projeto

```
carteira-inteligente/
├── Cargo.toml                   # Dependências e metadados do projeto
├── docker-compose.yml           # PostgreSQL pronto para ambiente de desenvolvimento
├── migrations/                  # Migrações SQL (Users, Assets, Transactions, Targets)
├── src/
│   ├── main.rs                  # Entrypoint do servidor Axum
│   ├── lib.rs                   # Re-export de módulos para testes
│   ├── config.rs                # Variáveis de ambiente (.env)
│   ├── db.rs                    # Pool de conexões SQLx (PostgreSQL / SQLite fallback)
│   ├── error.rs                 # Tratamento tipado de erros (AppError)
│   ├── auth/                    # Argon2id, Cookies e Middlewares de sessão
│   ├── models/                  # Structs de Usuários, Ativos, Transações e Carteira
│   ├── services/                # Regras de negócio, cotações e algoritmo de rebalanceamento
│   ├── routes/                  # Rotas web (HTML) e REST APIs (JSON)
│   └── templates/               # Templates Askama e filtros de formatação
├── templates/                   # Arquivos HTML compilados
└── static/                      # Arquivos estáticos (CSS moderno, JS e gráficos)
```
