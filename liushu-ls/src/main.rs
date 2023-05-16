use liushu_core::dirs::PROJECT_DIRS;
use liushu_core::engine::{Engine, InputMethodEngine};
use tokio::sync::{Mutex, RwLock};
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
    engine: Mutex<Engine>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        let engine = Engine::init(&PROJECT_DIRS).unwrap();
        Self {
            client,
            input: RwLock::new(String::new()),
            engine: Mutex::new(engine),
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
        let input = &self.input.read().await.clone();
        self.client
            .log_message(MessageType::INFO, format!("current input {:?}", input))
            .await;

        if input.is_empty() {
            return Ok(None);
        }

        match self.engine.lock().await.search(input) {
            Ok(list) => {
                let completion_resp = CompletionResponse::List(CompletionList {
                    is_incomplete: false,
                    items: list
                        .iter()
                        .map(|item| {
                            let label = format!(
                                "{} {}",
                                item.text,
                                item.comment.clone().unwrap_or(item.code.clone())
                            );

                            CompletionItem {
                                label,
                                sort_text: Some(item.code.clone()),
                                filter_text: Some(input.to_owned()),
                                insert_text: Some(item.text.clone()),
                                kind: Some(CompletionItemKind::TEXT),
                                ..Default::default()
                            }
                        })
                        .collect(),
                });
                Ok(Some(completion_resp))
            }
            Err(_) => Ok(None),
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

    let (service, socket) = LspService::new(Backend::new);

    Server::new(stdin, stdout, socket).serve(service).await;
}
