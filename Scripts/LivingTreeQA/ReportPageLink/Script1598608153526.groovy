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

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.RIYAS_USERNAME)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.verifyElementPresent(findTestObject('LivingTree/MyProfile/DropDownOption'), 0)

WebUI.click(findTestObject('LivingTree/MyProfile/DropDown'))

WebUI.verifyElementPresent(findTestObject('LivingTree/ReportPageLink/ReportsButton'), 0)

WebUI.click(findTestObject('LivingTree/ReportPageLink/ReportsButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ReportPageLink/ReportPageLeftSide'), 1)

WebUI.waitForElementPresent(findTestObject('LivingTree/ConversationPage/AllIcon'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/ReportPageLink/OverallSnapshotText'), 0)

WebUI.waitForElementPresent(findTestObject('LivingTree/ReportPageLink/SnapshotPAge'), 0)

WebUI.click(findTestObject('LivingTree/ReportPageLink/SchoolDropDown'))

WebUI.click(findTestObject('LivingTree/ReportPageLink/SchoolDropDown'))

WebUI.click(findTestObject('LivingTree/ReportPageLink/ViewButton'))

WebUI.waitForElementPresent(findTestObject('LivingTree/ReportPageLink/ViewPage'), 2)

WebUI.click(findTestObject('LivingTree/ReportPageLink/CloseButton'), FailureHandling.STOP_ON_FAILURE)

WebUI.closeBrowser()

