use liushu_core::dict::query_code;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[derive(Debug)]
struct Backend {
    client: Client,
    input: RwLock<String>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            input: RwLock::new(String::new()),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions::default()),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::INFO, "server shut down!")
            .await;
        Ok(())
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let input = self.input.read().await.clone();
        self.client
            .log_message(MessageType::INFO, format!("current input {:?}", input))
            .await;

        if input.is_empty() {
            Ok(None)
        } else {
            let input_ = &input.clone();
            Ok(Some(
                query_code(input, 1)
                    .map(|list| {
                        CompletionResponse::List(CompletionList {
                            is_incomplete: false,
                            items: list
                                .iter()
                                .map(move |result| CompletionItem {
                                    label: result.to_string(),
                                    detail: Some(result.to_string()),
                                    sort_text: Some(input_.to_string()),
                                    filter_text: Some(input_.to_string()),
                                    insert_text: Some(result.to_string()),
                                    kind: Some(CompletionItemKind::TEXT),
                                    ..Default::default()
                                })
                                .collect(),
                        })
                    })
                    .unwrap(),
            ))
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(new_input) = params.content_changes.get(0) {
            let re = regex!(r"[a-z]+");
            let mut input_writer = self.input.write().await;

            match (re.is_match(&input_writer), re.is_match(&new_input.text)) {
                (true, true) => {
                    (*input_writer).push_str(&new_input.text);
                }
                (false, true) => {
                    *input_writer = new_input.text.clone();
                }
                (_, false) => {
                    *input_writer = "".to_string();
                }
            };
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));

    Server::new(stdin, stdout, socket).serve(service).await;
}
