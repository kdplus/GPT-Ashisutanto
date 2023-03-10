import init, { print_upper_string, event_create } from "./gpt_ashisutanto.js";

chrome.runtime.onInstalled.addListener(function() {
  chrome.contextMenus.create({
    id: "sendToRust",
    title: "Create event in Google Calander",
    contexts: ["selection"]
  });
});

chrome.contextMenus.onClicked.addListener(function(info, tab) {
  if (info.menuItemId === "sendToRust") {
    // chrome.tabs.sendMessage(tab.id, {selectionText: info.selectionText}, function(response) {
    //   console.log(response);
    // });
    sendSelectionToEventCreate(info, tab);
  }
});

async function sendSelectionToEventCreate(info, tab) {
  const selectedText = info.selectionText;
  const result = await event_create(selectedText);
  console.log(result);
}

async function main() {
  await init();

  chrome.tabs.onActivated.addListener(async (tab) => {
    const activeTab = await chrome.tabs.get(tab.tabId);
    console.log(print_upper_string(activeTab.url));
  });
}

main();
