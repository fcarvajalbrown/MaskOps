# LATAM Privacy Law Reference

> Reference for MaskOps development and go-to-market planning.
> Covers the 8 primary jurisdictions relevant to the LATAM commercial opportunity.
> Last reviewed: 2026-06-03. Laws change — verify against official sources before legal reliance.

---

## Quick-Reference Table

| Country | Law | In Force | Regulator | Max Penalty | Data Localization | EU Adequacy |
|---------|-----|----------|-----------|-------------|-------------------|-------------|
| Brazil | LGPD (Lei 13.709/2018) | Aug 2020 | ANPD | 2% of BR revenue, cap BRL 50M/violation | No mandate — adequate safeguards required | No (under review) |
| Argentina | Ley 25.326 (PDPA) | Oct 2000 | APDP | ARS fines (reform pending) | No | Yes (since 2003) |
| Chile | Ley 19.628 + new bill (Aug 2024) | 1999 / pending enactment | CPDP (new) | UF 5,000–10,000 (~USD 220K–440K) proposed | No | Pending |
| Colombia | Ley 1581/2012 | Nov 2013 | SIC | Up to 2,000 SMMLV (~USD 600K) | No | No |
| Mexico | LFPDPPP (2010) + new FLPPI | Jul 2010 / new pending | INAI | MXN 320M (~USD 16M) proposed new law | No | No |
| Peru | Ley 29733 | Jul 2011 | APDP | Up to 100 UIT (~USD 140K) | No | No |
| Ecuador | LOPDP | Enforced May 2023 | SPDP | Up to USD 1M (critical infractions) | No | No |
| Uruguay | Ley 18.331 | Aug 2008 | URCDP | Fines + processing suspension | No | Yes (since 2012) |

**Key takeaway:** No LATAM country currently mandates hard data localization (data must not leave the country). All require adequate protection measures. The air-gapped value proposition is about *risk reduction and sales positioning*, not strict legal compliance.

---

## Brazil — LGPD

**Lei Geral de Proteção de Dados Pessoais (Lei 13.709/2018)**

- **Enforced:** August 2020. ANPD gained sanctioning power September 2021.
- **Regulator:** ANPD (Autoridade Nacional de Proteção de Dados)
- **Penalties:** Up to 2% of Brazilian gross revenue in the prior fiscal year, capped at BRL 50 million per violation. Non-monetary: public warning, data deletion order, partial/total processing ban.
- **Sensitive data:** Health data, biometrics, racial/ethnic origin, political opinion, religion, sexual orientation. Requires explicit consent or legal basis.
- **Data transfers:** Permitted to countries with adequate protection or using approved SCCs (standard contractual clauses approved by ANPD in 2024). No localization mandate.
- **Key 2024–2025 developments:** ANPD approved SCCs for international transfers; first significant enforcement actions issued; DPA required for high-risk processing.
- **Relevance to MaskOps:** LGPD drives demand for CPF and RUT masking in Brazilian financial/healthcare pipelines. Explicit data minimization obligations create direct demand for PII redaction tooling.
- **Official text (Portuguese):** https://www.planalto.gov.br/ccivil_03/_ato2015-2018/2018/lei/l13709.htm
- **ANPD:** https://www.gov.br/anpd/pt-br

---

## Argentina — Ley 25.326

**Ley de Protección de los Datos Personales (Ley 25.326/2000)**

- **Enforced:** October 2000. One of the oldest GDPR-equivalent laws in the world.
- **Regulator:** APDP (Agencia de Acceso a la Información Pública, formerly DNPDP)
- **Penalties:** Administrative fines. Reform bill in Congress (2025) proposes penalties up to 2% of annual revenue.
- **Sensitive data:** Health, financial, political, religious, union membership. Special protection regime.
- **Data transfers:** Permitted only to countries with adequate protection. Argentina itself has EU adequacy status.
- **Key 2024–2025 developments:** Comprehensive reform bill introduced; expected to modernize to GDPR-level standards. DNI (national ID) and CUIL/CUIT added to regulated sensitive identifiers in the reform.
- **Relevance to MaskOps:** Argentine DNI is a v1.3.0 roadmap item — the reform discussion makes it commercially urgent to move earlier (v0.5.0 proposed).
- **Official text (Spanish):** https://servicios.infoleg.gob.ar/infolegInternet/anexos/60000-64999/64790/norma.htm
- **APDP:** https://www.argentina.gob.ar/aaip/datospersonales

---

## Chile — Ley 19.628 + New Bill

**Current: Ley sobre Protección de la Vida Privada (Ley 19.628/1999)**
**Pending: New Data Protection Bill (passed Congress Aug 2024)**

- **Enforced:** 1999 (current). New bill pending constitutional court review → presidential enactment.
- **Regulator (new):** CPDP (Consejo para la Transparencia will be replaced by a dedicated DPA).
- **Penalties (proposed):** UF 1,000–10,000 (~USD 44K–440K) depending on severity. Critical violations: up to UF 10,000.
- **Sensitive data:** Health, biometrics, union, political opinion, religion. New bill adds genetic and sexual orientation data.
- **Data transfers:** New bill aligns with GDPR — requires adequacy or appropriate safeguards. No localization mandate.
- **Key 2024–2025 developments:** This is the most significant LATAM legislative event of 2024. The new law will introduce DPOs, data breach notification (72h), and explicit rights (access, rectification, erasure, portability).
- **Relevance to MaskOps:** Chile is the home jurisdiction of MaskOps' author. RUT masking is already implemented. The new law creates direct commercial demand from Chilean financial institutions, ISAPREs (health insurers), AFPs (pension funds), and municipalities.
- **Official current text (Spanish):** https://www.bcn.cl/leychile/navegar?idNorma=141599
- **New bill tracking:** https://www.camara.cl/pley/pley_detalle.aspx?prmID=16919

**Additional Chilean regulatory layer — CMF NCG 311:**
Entities regulated by the Comisión para el Mercado Financiero (banks, fintechs, payment processors, AFPs, ISAPREs) must comply with CMF NCG 311, which mandates cybersecurity and data governance standards for financial data. MaskOps directly supports NCG 311 compliance for these clients.

**ISO certification relevance for Chile:**
- **ISO 27701** is the most defensible proof of compliance with Ley 21.719 for a service provider. Required investment: USD 15–40K + 6–12 months. Relevant only at Phase 5 (government procurement via Mercado Público).
- **Mercado Público** (Chilean government procurement platform) frequently requires ISO 27001 and ISO 27701 as mandatory prerequisites for technology bids. Pursue only after 2+ reference clients.
- **Sales angle now:** *"MaskOps is built to help your team achieve ISO 27701 compliance"* — accurate and effective without any certification required on your part.

---

## Colombia — Ley 1581

**Ley Estatutaria 1581/2012 de Protección de Datos Personales**

- **Enforced:** November 2013.
- **Regulator:** SIC (Superintendencia de Industria y Comercio)
- **Penalties:** Up to 2,000 SMMLV (salario mínimo mensual legal vigente) — approximately USD 600K at 2025 rates. Temporary processing suspension also available.
- **Sensitive data:** Racial/ethnic origin, political opinion, religious beliefs, union membership, health/sex life data. Prohibited processing without explicit consent.
- **Data transfers:** Permitted to countries with adequate protection (Colombia maintains its own adequacy list).
- **Key 2024–2025 developments:** SIC has increased enforcement frequency. Reform bill in Congress proposes expanding law to cover AI-driven profiling and automated decision-making.
- **Relevance to MaskOps:** Colombian CC (cédula de ciudadanía) and NIT are the target identifiers. SIC's enforcement activity validates the pipeline compliance market.
- **Official text (Spanish):** https://www.funcionpublica.gov.co/eva/gestornormativo/norma.php?i=49981
- **SIC:** https://www.sic.gov.co/

---

## Mexico — LFPDPPP

**Ley Federal de Protección de Datos Personales en Posesión de los Particulares (2010)**

- **Enforced:** July 2010.
- **Regulator:** INAI (Instituto Nacional de Transparencia, Acceso a la Información y Protección de Datos Personales)
- **Penalties:** Up to MXN 320 million (~USD 16M) under proposed new Federal Law on Personal Data Protection (FLPPI, in Congress 2025).
- **Sensitive data:** Health, biometric, genetic, racial/ethnic, religious, sexual orientation.
- **Data transfers:** Permitted with consent or contractual safeguards. No localization mandate.
- **Key 2024–2025 developments:** FLPPI (new law) in legislative process — would replace the 2010 law with GDPR-equivalent obligations including DPOs, breach notification, and automated decision rights. CURP is a regulated identifier.
- **Relevance to MaskOps:** CURP masking already implemented. New law raises stakes for organizations processing CURP, RFC, and CLABE in batch pipelines.
- **Official current text (Spanish):** https://www.diputados.gob.mx/LeyesBiblio/pdf/LFPDPPP.pdf
- **INAI:** https://home.inai.org.mx/

---

## Peru — Ley 29733

**Ley de Protección de Datos Personales (Ley 29733/2011)**

- **Enforced:** July 2011. Regulations (DS 003-2013-JUS) in force March 2013.
- **Regulator:** APDP (Autoridad Nacional de Protección de Datos Personales), under the Ministry of Justice.
- **Penalties:** Up to 100 UIT (Unidad Impositiva Tributaria) per violation — approximately USD 140K at 2025 rates. Aggravating factors can increase this.
- **Sensitive data:** Biometric, health, ideology, political opinion, religion, trade union membership, sexual life.
- **Data transfers:** Permitted with consent or to countries with adequate protection.
- **Key 2024–2025 developments:** APDP has accelerated enforcement in healthcare and financial sectors. Discussion of reform to include AI and algorithmic profiling.
- **Relevance to MaskOps:** DNI peruano (Documento Nacional de Identidad) is an 8-digit number — a roadmap target. Peruvian healthcare and financial firms are a secondary LATAM commercial target.
- **Official text (Spanish):** https://lpderecho.pe/ley-proteccion-datos-personales-ley-29733-actualizada/
- **APDP:** https://www.minjus.gob.pe/privacidad/

---

## Ecuador — LOPDP

**Ley Orgánica de Protección de Datos Personales (2021)**

- **Enacted:** May 2021. **Fully enforceable:** May 2023 (2-year transition period).
- **Regulator:** SPDP (Superintendencia de Protección de Datos Personales) — established 2023.
- **Penalties:** Light infractions: up to USD 10K. Serious: USD 10K–100K. Critical: USD 100K–1M or 1% of annual revenue, whichever is higher.
- **Sensitive data:** Health, biometric, genetic, racial/ethnic, religion, political opinion, sexual orientation. Minors' data gets extra protection.
- **Data transfers:** Permitted to countries with adequate protection or with explicit consent and contractual safeguards.
- **Key 2024–2025 developments:** SPDP issued first enforcement actions in 2024. Ecuador is the most recently activated regulator in the region — enforcement posture still developing.
- **Relevance to MaskOps:** Ecuador's cedula de identidad (10-digit) is an unimplemented identifier. Growing healthcare and financial sector compliance demand.
- **Official text (Spanish):** https://www.telecomunicaciones.gob.ec/wp-content/uploads/2021/06/Ley-Organica-Proteccion-Datos-Personales.pdf
- **SPDP:** https://www.spdp.gob.ec/

---

## Uruguay — Ley 18.331

**Ley de Protección de Datos Personales y Acción de Habeas Data (Ley 18.331/2008)**

- **Enforced:** August 2008. Decree 414/009 (2009) adds regulations.
- **Regulator:** URCDP (Unidad Reguladora y de Control de Datos Personales)
- **Penalties:** Fines + temporary or permanent processing suspension. Specific amounts set per case.
- **Sensitive data:** Racial/ethnic origin, political opinion, religion, trade union, sexual orientation, health.
- **Data transfers:** Permitted only to adequate-protection countries. Uruguay has EU adequacy status (2012).
- **Key 2024–2025 developments:** Reform discussions underway to align with GDPR 2.0 developments. URCDP remains one of the more active regulators in the region.
- **Relevance to MaskOps:** Uruguay's cédula de identidad is an 8-digit number. Uruguay's dual EU adequacy status makes it a bridge jurisdiction for LATAM–EU data pipelines — MaskOps' EU + LATAM coverage is directly relevant.
- **Official text (Spanish):** https://www.impo.com.uy/bases/leyes/18331-2008
- **URCDP:** https://www.gub.uy/unidad-reguladora-control-datos-personales/

---

## Common Threads Across All 8 Jurisdictions

1. **No hard data localization mandate in any country.** "Data sovereignty" claims in sales materials are about risk reduction, not legal requirements.
2. **All follow GDPR-adjacent principles:** purpose limitation, data minimization, explicit consent for sensitive data, breach notification.
3. **Sensitive data categories are consistent:** health, biometric, financial, political, religious. All have stricter processing rules — these are exactly the data types MaskOps protects.
4. **Enforcement is accelerating:** ANPD (Brazil), SIC (Colombia), SPDP (Ecuador), and CPDP (Chile, new) are all increasing enforcement frequency and penalty amounts.
5. **Healthcare and financial sectors are the primary enforcement targets** across all 8 countries — consistent with the MaskOps target demographics.

---

## Identifiers by Jurisdiction (MaskOps Coverage Status)

| Country | Primary ID | Format | MaskOps Status |
|---------|-----------|--------|----------------|
| Brazil | CPF | 11 digits (Módulo 11) | ✓ Implemented |
| Chile | RUT | 8-9 digits + check | ✓ Implemented |
| Mexico | CURP | 18-char alphanumeric | ✓ Implemented |
| Argentina | DNI | 8 digits | Roadmap v0.5.0 |
| Colombia | Cédula CC / NIT | 6–10 digits | Roadmap v0.5.0 |
| Peru | DNI | 8 digits | Roadmap v1.3.0 |
| Ecuador | Cédula | 10 digits (Módulo 10) | Not scheduled |
| Uruguay | Cédula | 7–8 digits | Not scheduled |

---

*Sources: ANPD, APDP, CPDP bill tracking, SIC, INAI, APDP Peru, SPDP, URCDP, Crowell & Moring LATAM Privacy summary, TrustArc LATAM 2025 report, FPF LatAm DPA Report 2024.*
