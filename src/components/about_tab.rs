// Copyright 2024, Alan Sparrow
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//
use leptos::html::{a, address, div, em, p};
use leptos::prelude::*;

pub fn about_tab() -> impl IntoView {
    div().class("content").child((
        div().class("subtitle").child("Output Format"),
        p().child(
            "ASSelect generates OpenAir format airspace data for the mainland UK."
        ),
        div().class("subtitle").child("Airspace Types"),
        p().child((
            em().child("Non-ATZ Airfields: "),
            "Busy training aerodromes (without an ATZ) as listed in section \
             ENR\u{00a0}5.5 of the AIP. The AIP does not include the \
             majority of smaller airfields shown on the 1:500000 chart, and \
             ASSelect doesn't include them either.",
        )),
        p().child((
            em().child("Gliding Airfields: "),
            "Listed in ENR\u{00a0}5.5. ASSelect plots \
             them with a radius of 1\u{00a0}nm. ASSelect can optionally \
             exclude your home site to avoid unnecessary airspace warnings.",
        )),
        p().child((
            em().child("Microlight Airfields "),
            "Listed in ENR\u{00a0}5.5 and plotted with a radius of 0.5\u{00a0}nm.",
        )),
        p().child((
            em().child("Obstacles: "),
            "From ENR\u{00a0}5.4. Only obstacles with a height of greater than \
             600\u{00a0}ft (mainly radio masts) are included.",
        )),
        p().child((
            em().child("ILS Feathers: "),
            "Not strictly an airspace type, but the BGA recommend a radio \
             call if you fly in their vicinity. The feather is \
             symbolic only - the actual instrument approach procedure \
             will extend to one side of the feather.",
        )),
        div().class("subtitle").child("Local Agreements"),
        p().child((
            "BGA letters of agreement are described in detail on the ",
            a().class("text-primary")
                .href("https://members.gliding.co.uk/library/loas/")
                .child("BGA website"),
            ". A local club briefing is required before using any local agreement \
             (Except Cambridge RAZ.)",
        )),
        div().class("subtitle").child("Radio Frequencies"),
        p().child(
            "Radio frequencies are stored using the OpenAir \"AF\" record \
            type. Optionally ASSelect can append them to the airspace name.",
        ),
        div()
            .class("subtitle")
            .child("Temporary Restrictions, RA(T)"),
        p().child((
            "RA(T)s are detailed in Mauve AICs on the ",
            a().class("text-primary")
                .href("https://nats-uk.ead-it.com/cms-nats/opencms/en/Publications/Aeronautical-Information-Circulars-AICs/mauve-aics/")
                .child("NATS AIS website"),
            ". RA(T)s can be downloaded separately from the rest of the airspace.",
        )),
        div().class("subtitle").child("Altitude Overlay"),
        p().child(
            "ASSelect can generate a graphical overlay showing the base of \
             controlled airspace. The overlay is encoded as Class B airspace."),
        div().class("subtitle").child("Data"),
        p().child((
            "Airspace data is updated every four weeks - see ",
            a().class("text-primary")
                .href("https://nats-uk.ead-it.com/cms-nats/export/sites/default/en/Publications/publication-schedule/10-year-AIRAC.pdf")
                .child(" Schedule "),
            ". This site is normally updated approximately two weeks in advance of \
             the effective date",)),
        p().child((
            "The ASSelect airspace database can be downloaded from ",
            a().class("text-primary").href("https://github.com/ahsparrow/airspace").child("GitHub"),
            ". (Also ",
            a().class("text-primary").href("https://github.com/ahsparrow/asselect3").child("GitHub"),
            " for website source.)",)),
         div().class("subtitle").child("Contact"),
         address().child((
            "Comments, corrections and complaints to: ",
            a().href("mailto:web@asselect.uk").child("Alan Sparrow"),)),
    ))
}
