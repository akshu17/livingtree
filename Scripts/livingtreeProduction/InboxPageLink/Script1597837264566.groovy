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

WebUI.click(findTestObject('LivingTree/ConversationPageLink/InboxIcon'))

WebUI.waitForElementPresent(findTestObject('LivingTree/InboxcStructureCheck/InboxCreatePage'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/InboxcStructureCheck/InboxLeftSide'), 0)

WebUI.click(findTestObject('LivingTree/InboxcStructureCheck/InboxLeftSide'))

WebUI.waitForElementPresent(findTestObject('LivingTree/InboxcStructureCheck/LeftSideDrafts'), 0)

WebUI.click(findTestObject('LivingTree/InboxcStructureCheck/LeftSideDrafts'))

WebUI.waitForElementPresent(findTestObject('LivingTree/InboxcStructureCheck/LeftSideSentMessages'), 0)

WebUI.click(findTestObject('LivingTree/InboxcStructureCheck/LeftSideSentMessages'))

WebUI.waitForElementPresent(findTestObject('LivingTree/InboxcStructureCheck/InboxLeftSide'), 0)

WebUI.click(findTestObject('LivingTree/InboxcStructureCheck/InboxLeftSide'))

WebUI.waitForElementPresent(findTestObject('LivingTree/InboxcStructureCheck/DeleteButton'), 0)

WebUI.check(findTestObject('LivingTree/InboxLink/CheckBox'))

WebUI.click(findTestObject('LivingTree/InboxLink/DeleteButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/InboxcStructureCheck/ReadButton'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/InboxcStructureCheck/ToggleButton'), 0)

WebUI.click(findTestObject('LivingTree/InboxcStructureCheck/ToggleButton'))

WebUI.click(findTestObject('LivingTree/InboxcStructureCheck/ToggleButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/InboxcStructureCheck/UnreadButton'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/InboxcStructureCheck/InboxNumber'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/InboxLink/ComposeMessageButton'), 
    0)

WebUI.click(findTestObject('LivingTree/InboxLink/ComposeMessageButton'))

WebUI.click(findTestObject('LivingTree/InboxLink/SaveDraftButton'))

WebUI.closeBrowser()

