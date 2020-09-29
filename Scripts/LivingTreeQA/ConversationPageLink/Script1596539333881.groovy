import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser(GlobalVariable.URL)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.EMAIL_TEACHER)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/LivingTreeIcon'), 1)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/LivingTreeIcon'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/ConversationPageAllIcon'), 1)

WebUI.click(findTestObject('LivingTree/ConversationPage/ConversationIconCoversationPage'))

WebUI.delay(3)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPage/CalendarIconConversationPage'), 0)

WebUI.delay(2)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPage/GiveIconConversationPage'), 0)

WebUI.click(findTestObject('LivingTree/ConversationPage/CalendarIconConversationPage'))

WebUI.delay(2)

WebUI.waitForElementPresent(findTestObject('LivingTree/GivePage/GivePageIcon'), 1)

WebUI.click(findTestObject('LivingTree/GivePage/GivePageIcon'))

WebUI.delay(2)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/InboxIcon'), 0)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/InboxIcon'))

WebUI.delay(2)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/ConversationPageMyNetworkIcon'), 0)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/ConversationPageMyNetworkIcon'))

WebUI.delay(2)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/ConversationPageDirectoryIcon'), 0)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/ConversationPageDirectoryIcon'))

WebUI.delay(2)

WebUI.click(findTestObject('LivingTree/ConversationPage/ConversationIconCoversationPage'))

WebUI.delay(2)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/ShowAllTypeButton'), 2)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/label_Users'))

WebUI.waitForPageLoad(4)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/button_Close1'))

WebUI.click(findTestObject('LivingTree/ConversationPageLink/LabelAll'))

WebUI.waitForPageLoad(4)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/LabelPinned'))

WebUI.waitForPageLoad(4, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/LabelFiles'))

WebUI.waitForPageLoad(4, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/LabelPhotosVideos'))

WebUI.waitForPageLoad(4, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/LabelEvents'))

WebUI.waitForPageLoad(4)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/GROUPFILTERButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/GroupFilterText'), 0)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/ClearFilterButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/FAMILYButton'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/AddChildButton'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/GROUPSButton'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/Intercom'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/RightSideIcon'), 0)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/WhatsNewBox'))

WebUI.click(findTestObject('LivingTree/ConversationPage/SendUegentButton'))

WebUI.click(findTestObject('LivingTree/ConversationPage/CreatePostCloseButton'))

WebUI.waitForElementClickable(findTestObject('LivingTree/ConversationPage/ConfigureDefaultMessageBox'), 3)

WebUI.click(findTestObject('LivingTree/ConversationPage/ConfigureDefaultMessageBox'))

WebUI.delay(10)

WebUI.verifyElementPresent(findTestObject('LivingTree/ConversationPage/DefaultUrgentAlertMessageBox'), 0)

WebUI.click(findTestObject('LivingTree/ConversationPage/DefaultUrgentAlertMessageCloseBox'))

WebUI.delay(2)

WebUI.click(findTestObject('Page_LivingTree teacher taa/span_Add Member'))

WebUI.delay(1)

WebUI.click(findTestObject('LivingTree/MyNetworkPageLink/MyNetworkInvitePopUpParentBox'))

WebUI.delay(2)

WebUI.waitForElementVisible(findTestObject('LivingTree/ConversationPageLink/TodayConversationBox'), 1)

WebUI.waitForElementVisible(findTestObject('Edlio/ConversationPage/SocialMediaLinkAppStore'), 0)

WebUI.waitForElementVisible(findTestObject('Edlio/ConversationPage/SocialMediaLinkPlayStore'), 0)

WebUI.closeBrowser()

