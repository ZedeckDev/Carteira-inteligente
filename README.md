# Carteira Inteligente

Aplicação web de gestão e rebalanceamento de investimentos, construída em Rust/Axum com PostgreSQL.

## Publicar na web — sem terminal

A branch contém um `render.yaml` e um `Dockerfile` para publicar pelo painel do Render.

1. Abra o [Render Dashboard](https://dashboard.render.com/) e escolha **New > Blueprint**.
2. Conecte o repositório e selecione a branch que contém `render.yaml`.
3. Confirme a criação do serviço web e do PostgreSQL; o Render conecta `DATABASE_URL` automaticamente.
4. Após o deploy, abra a URL `.onrender.com` fornecida pela plataforma.

O serviço responde a `/api/health` para verificação de disponibilidade. A aplicação usa a porta definida pela plataforma, sem exigir qualquer comando do usuário final.

## Variáveis de ambiente

| Variável | Uso |
| --- | --- |
| `DATABASE_URL` | Conexão PostgreSQL gerenciada |
| `HOST` | Interface de escuta; em produção, `0.0.0.0` |
| `PORT` | Porta HTTP configurada pela hospedagem |
| `SESSION_SECRET` | Reservada para a configuração de sessão |

## Funcionalidades

- Cadastro e login.
- Cadastro de ativos e lançamento de compras, vendas e proventos.
- Dashboard de posição, resultado e alocação.
- Atualização de cotações e simulador de aporte/rebalanceamento.

## Estrutura de deploy

- `Dockerfile`: build multiestágio do binário Rust e imagem final enxuta.
- `render.yaml`: serviço web, health check e PostgreSQL conectados.
- `static/`: estilos e scripts que as páginas já referenciavam, agora publicados pelo Axum.
