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

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPageEmailAdressTextBox'), GlobalVariable.EMAIL_PRINCIPLE)

WebUI.setText(findTestObject('LivingTree/LoginPage/LoginPagePasswordTextBox'), GlobalVariable.PASSWORD)

WebUI.click(findTestObject('LivingTree/LoginPage/LoginPageLoginButton'))

WebUI.click(findTestObject('LivingTree/MyNetworkPage/MyNetworkPageIcon'))

WebUI.verifyElementPresent(findTestObject('LivingTreeProduction/MyNetworkPage/SchoolofIndia'), 0)

WebUI.click(findTestObject('LivingTreeProduction/MyNetworkPage/SchoolofIndia'))

WebUI.delay(5)

WebUI.waitForElementVisible(findTestObject('LivingTreeProduction/MyNetworkPage/DailalogBoxEditPencilLivingtreeProd'), 2)

WebUI.delay(1)

WebUI.mouseOver(findTestObject('LivingTreeProduction/MyNetworkPage/DailalogBoxEditPencilLivingtreeProd'))

WebUI.click(findTestObject('LivingTreeProduction/MyNetworkPage/DailalogBoxEditPencilLivingtreeProd'))

WebUI.delay(1)

WebUI.waitForElementPresent(findTestObject('LivingTree/NetworkPageDailogBox/DialogBox'), 2)

WebUI.closeBrowser()

