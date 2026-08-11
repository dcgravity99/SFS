from orchestrator import SIRAOrchestrator


sira = SIRAOrchestrator()

sira.status()


sira.execute(
[
"story",
"screenwriter",
"director",
"camera",
"producer",
"editor"
]
)
