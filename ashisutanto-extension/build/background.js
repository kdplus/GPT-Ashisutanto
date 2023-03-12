import wasm_init, { event_create } from "./gpt_ashisutanto.js";


chrome.runtime.onInstalled.addListener(function() {
  chrome.contextMenus.create({
    id: "createEvent",
    title: "Create event in Google Calendar",
    contexts: ["selection"]
  });
});

chrome.contextMenus.onClicked.addListener(function(info) {
  if (info.menuItemId === "createEvent") {
    sendSelectionToEventCreate(info);
  }
});

async function sendSelectionToEventCreate(info) {
  const selectedText = info.selectionText;
  const gptAnswer = await event_create(selectedText);
  console.log(gptAnswer);
  createGoogleCalendarEvent(gptAnswer);
}

function createGoogleCalendarEvent(eventInfo) {

  // Parse the time duration string
  const regex = /!#\?(.+?)\?#!/g;
  const matches = regex.exec(eventInfo.time);
  const timeStr = matches[1];
  const timeZone = 'Asia/Tokyo';
  const [startDateStr, startTimeStr, endTimeStr] = timeStr.split(/[\s~]+/);
  const [startMonth, startDay, startYear] = startDateStr.split("/");
  const [startHour, startMinute] = startTimeStr.split(":");
  const [endHour, endMinute] = endTimeStr.split(":");
  const [endMonth, endDay, endYear] = startDateStr.split("/");

  const event = {
    summary: eventInfo.text,
    location: 'Event Location',
    description: eventInfo.detail,
    start: {
      dateTime: new Date(startYear, startMonth - 1, startDay, startHour, startMinute).toISOString(),
      timeZone: timeZone
    },
    end: {
      dateTime: new Date(endYear, endMonth - 1, endDay, endHour, endMinute).toISOString(),
      timeZone: timeZone
    },
  };

  // console.log(event);
  let url = "https://calendar.google.com/calendar/u/0/r/eventedit?";
  url += "dates=" + event.start.dateTime.replace(/[^0-9a-z]/gi, '') + "/" + event.end.dateTime.replace(/[^0-9a-z]/gi, '') + "&";
  url += "text=" + event.summary + "&";
  url += "location=" + event.location + "&";
  url += "details=" + event.description + "&";
  window.open(url, '_blank');
}

async function main() {
  await wasm_init();
}

main();
