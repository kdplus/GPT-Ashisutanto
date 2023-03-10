import wasm_init, { event_create } from "./gpt_ashisutanto.js";

chrome.runtime.onInstalled.addListener(function() {
  chrome.contextMenus.create({
    id: "createEvent",
    title: "Create event in Google Calander",
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
  const result = await event_create(selectedText);

  // Print ChatGPT reply 
  console.log(result);
}

async function main() {
  await wasm_init();
}

main();
