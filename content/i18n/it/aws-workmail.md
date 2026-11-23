# aws workmail

Gestisce Amazon WorkMail.

## Usage
- `aws workmail list-organizations` : elenca tutte le organizzazioni WorkMail
- `aws workmail list-users --organization-id id_organizzazione` : elenca tutti gli utenti di un'organizzazione specifica
- `aws workmail create-user --name nome_utente --display-name name --password password --organization-id id_organizzazione` : crea un utente WorkMail in un'organizzazione specifica
- `aws workmail register-to-work-mail --entity-id id_entita' --email email --organization-id id_organizzazione` : registra e abilita un gruppo/utente per WorkMail
- `aws workmail create-group --name group_name --organization-id id_organizzazione` : crea un gruppo WorkMail in un'organizzazione specifica
- `aws workmail associate-member-to-group --group-id group_id --member-id member_id --organization-id id_organizzazione` : associa un membro a un gruppo specifico
- `aws workmail deregister-from-work-mail --entity-id id_entita' --organization-id id_organizzazione` : deregistra e disabilita un utente/gruppo da WorkMail
- `aws workmail delete-user --user-id user_id --organization-id id_organizzazione` : elimina un utente da un'organizzazione

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/workmail/
