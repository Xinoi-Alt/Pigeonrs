# Workflow für Pigeonrs

```
Projekt-Eintrag  →  Issue  →  Branch  →  Pull Request  →  Merge
 (Entwurf)         (Label,    (ID +       (Reviewer,       (Issue schließt,
                   Meilenst.)  Name)       Closes #nr)      Board → Done)
```

---

## 1. Projekt-Eintrag übernehmen und in ein Issue umwandeln

1. Den Eintrag im Projekt-Board öffnen.
2. **Assignees:** sich selbst eintragen.
3. **Status:** auf `In Progress` setzen.
4. Über das **…**-Menü des Eintrags **Convert to issue** wählen und unser Repo auswählen.

Nur Aufgaben, an denen gerade gearbeitet wird, werden zu Issues. Alles andere bleibt Entwurf.

## 2. Issue mit Label und Meilenstein versehen

Im Issue rechts in der Seitenleiste:

- **Labels:** ein Typ-Label
- **Milestone:** den Meilenstein der Aufgabe, z. B. `M2 Chunking & Resume`

Der Issue-Titel bleibt so, wie er aus dem Board kommt, also mit der Aufgaben-ID vorne, z. B. `S3-02 Offer- und Accept-Nachrichten`.

## 3. Branch erstellen

**Name:** `<ID>-<kurzname>`, ohne Typ davor. Die ID bleibt wie im Board, der Kurzname ist klein geschrieben, Wörter mit `-` getrennt.

| Beispiel | Aufgabe |
| --- | --- |
| `S3-02-offer-accept` | S3-02 Offer- und Accept-Nachrichten |
| `S4-05-test-proxy` | S4-05 Test-Proxy |
| `S5-03-geaenderte-quelldatei` | S5-03 Geänderte Quelldatei erkennen |

**Am besten direkt aus dem Issue:** Rechts im Issue unter **Development** auf **Create a branch** klicken, den Namen wie oben eintragen und **Create branch** wählen. Der Branch ist dann automatisch mit dem Issue verknüpft. Danach lokal:

```bash
git fetch origin
git checkout S3-02-offer-accept
```

Oder alles auf einmal mit der GitHub CLI (`12` = Nummer des Issues):

```bash
gh issue develop 12 --name S3-02-offer-accept --checkout
```

**Commits:** mit der Aufgaben-ID vorne, z. B. `S3-02: Offer-Nachricht serialisieren`.

### Commits
- Jeden Commit mit dem zugehörigen Issue verlinken:
'''markdown
"S3-02: Offer-Nachricht serialisieren (#12)"
'''

## 4. Pull Request öffnen

**Im PR ausfüllen:**

- **Titel:** wie das Issue, z. B. `S3-02 Offer- und Accept-Nachrichten`
- **Beschreibung:** `Closes #12` in die erste Zeile, als Sicherheit zusätzlich zur Verknüpfung über den Branch. Dazu kurz, was geändert wurde und wie man es testet (Vorlage unten).
- **Reviewers:** die Person, die laut Rotationsplan im nächsten Sprint diesen Bereich übernimmt
- **Labels** und **Milestone:** dieselben wie im Issue
- **Projects:** Pigeonrs (passiert automatisch, wenn der Workflow „Auto-add to project“ aktiv ist)

Mit der GitHub CLI:

```bash
gh pr create \
  --title "S3-02 Offer- und Accept-Nachrichten" \
  --body "Closes #12" \
  --reviewer <github-name> \
  --label "typ:feature,bereich:kern" \
  --milestone "M2 Chunking & Resume"
```

**Vorlage für die Beschreibung:**

```markdown
Closes #12

## Inhalt
...
```

## 5. Review und Merge

## Meilensteine

| Meilenstein | Fällig |
| --- | --- |
| M0 Setup | 01.12.2026 |
| M1 Ende-zu-Ende | 08.12.2026 |
| M2 Chunking & Resume | 19.01.2027 |
| M3 Verschlüsselung | 02.02.2027 |
| M4 Discovery & Robustheit | 09.02.2027 |
| M5 Feinschliff / App | 16.02.2027 |
| Abschluss | 01.03.2027 |
