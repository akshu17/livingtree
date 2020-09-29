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

WebUI.click(findTestObject('LivingTree/GivePage/GivePageIcon'))

WebUI.click(findTestObject('LivingTree/ConversationPageLink/GROUPFILTERButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/GroupFilterText'), 0)

WebUI.click(findTestObject('LivingTree/ConversationPageLink/ClearFilterButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/FAMILYButton'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/AddChildButton'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/GROUPSButton'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/Intercom'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPageLink/RightSideIcon'), 0)

WebUI.delay(2)

WebUI.waitForElementVisible(findTestObject('LivingTree/ConversationPageLink/TodayConversationBox'), 0)

WebUI.waitForElementVisible(findTestObject('Edlio/ConversationPage/SocialMediaLinkAppStore'), 0)

WebUI.waitForElementVisible(findTestObject('Edlio/ConversationPage/SocialMediaLinkPlayStore'), 0)

WebUI.waitForPageLoad(6)

WebUI.closeBrowser()

