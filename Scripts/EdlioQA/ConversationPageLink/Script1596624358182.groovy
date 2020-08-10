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

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/LabelAll'), 1)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/LabelAll'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/LabelEvents'), 1)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/LabelEvents'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/LabelFiles'), 1)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/LabelFiles'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/LabelPhotosVideos'), 2)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/LabelPhotosVideos'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/LabelPinned'), 1)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/LabelPinned'))

WebUI.click(findTestObject('LivingTree/ConversationPageLink/LabelUsers'))

WebUI.click(findTestObject('LivingTree/ConversationPageLink/UserCloseButton'))

WebUI.delay(5)

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

WebUI.click(findTestObject('LivingTree/ConversationPageLink/AddFamilyButton'))

WebUI.waitForElementVisible(findTestObject('LivingTree/ConversationPageLink/AddFamilyButtonClose'), 2)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/AddFamilyButtonClose'))

WebUI.delay(2)

WebUI.click(findTestObject('LivingTree/MyNetworkPage/AddGroupButton'))

WebUI.delay(1)

WebUI.click(findTestObject('Page_LivingTree teacher taa/button_Close'))

WebUI.delay(2)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/AddChildButton'))

WebUI.waitForElementVisible(findTestObject('LivingTree/ConversationPageLink/AddChildBox'), 0)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/AddChildButtonClose'))

